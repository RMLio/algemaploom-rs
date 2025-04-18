pub mod error;
mod operators;
pub mod parser;
mod types;
mod util;

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use error::RMLTranslationError;
use operator::{Extend, Operator};
use operators::projection::ProjectionTranslator;
use operators::source::SourceOpTranslator;
use parser::extractors::{rcterm_to_string, TermMapExtractor};
use parser::rml_model::term_map::SubjectMap;
use parser::rml_model::{Document, PredicateObjectMap, TriplesMap};
use plangenerator::data_type::RcRefCellPlan;
use plangenerator::error::PlanError;
use plangenerator::states::join::join;
use plangenerator::states::Processed;
use plangenerator::Plan;
use util::extract_tm_infos_from_sm_poms;

use self::operators::extend::*;
use self::operators::fragment::FragmentTranslator;
use self::operators::serializer::{self, translate_serializer_op};
use self::util::generate_lt_quads_from_spo;
use crate::rml::parser::extractors::io::parse_file;
use crate::rml::types::SearchMap;
use crate::rml::util::{
    generate_logtarget_map, generate_lt_quads_from_doc, generate_variable_map,
};
use crate::{LanguageTranslator, OperatorTranslator};

pub struct OptimizedRMLDocumentTranslator;

impl LanguageTranslator<&Path> for OptimizedRMLDocumentTranslator {
    fn translate_to_plan(path: &Path) -> crate::LanguageTranslateResult {
        let doc = parse_file(path.to_path_buf())?;
        Self::translate_to_plan(doc)
    }
}

impl LanguageTranslator<Document> for OptimizedRMLDocumentTranslator {
    fn translate_to_plan(doc: Document) -> crate::LanguageTranslateResult {
        let base_iri = doc.default_base_iri.clone();
        let mut plan = Plan::<()>::new();

        //For each triples maps, create a plan with source operator applied
        // Search dictionaries instantiations
        let variable_map = generate_variable_map(&doc);
        let target_map = generate_logtarget_map(&doc);
        let lt_id_quad_map = generate_lt_quads_from_doc(&doc);
        let tm_sourced_pairs: Vec<_> = doc
            .triples_maps
            .iter()
            .map(|tm| {
                let other_tms = doc
                    .triples_maps
                    .iter()
                    .filter(|other_tm| other_tm.identifier != tm.identifier)
                    .collect();
                let source_op =
                    SourceOpTranslator { tm, other_tms }.translate();
                //  let projection_op =
                //      translate_projection_op(tm, doc.triples_maps.iter());

                (
                    tm,
                    Rc::new(RefCell::new(
                        plan.source(source_op),
                        //              .apply(&projection_op, "Projection")?,
                    )),
                )
            })
            .collect();
        let tm_rccellplan_map: HashMap<_, _> = tm_sourced_pairs
            .clone()
            .into_iter()
            .map(|(tm, rccellplan)| (tm.identifier.clone(), (tm, rccellplan)))
            .collect();

        let search_map = SearchMap {
            tm_rccellplan_map,
            variable_map,
            target_map,
            lt_id_quad_map,
        };
        // Finish search dictionaries instantiations

        //Partition the previously generated plans, with triples maps,
        //to those with parent triples map
        //and those without (for handling joins)
        let (ptm_tm_plan_pairs, noptm_tm_plan_pairs): (Vec<_>, Vec<_>) =
            tm_sourced_pairs
                .into_iter()
                .partition(|(tm, _)| tm.contains_ptm());

        // Handle triples map with joins
        ptm_tm_plan_pairs.iter().try_for_each(|(tm, plan)| {
            let sm_ref = &tm.subject_map;
            let poms = tm.po_maps.clone();

            //Further separate POMs involved in joins and those uninvolved in joins
            let (joined_poms, no_join_poms): (Vec<_>, Vec<_>) =
                partition_pom_join_nonjoin(poms);

            if !joined_poms.is_empty() {
                add_join_related_ops(
                    tm,
                    &joined_poms,
                    sm_ref,
                    &search_map,
                    plan,
                    &base_iri,
                )?;
            }

            if !no_join_poms.is_empty() {
                add_non_join_related_ops(
                    &tm.identifier,
                    &no_join_poms,
                    sm_ref,
                    &search_map,
                    plan,
                    &base_iri,
                )?;
            }
            Ok::<(), RMLTranslationError>(())
        })?;

        // Simple case of triples maps without parent triples maps
        noptm_tm_plan_pairs.iter().try_for_each(|(tm, plan)| {
            let sm_ref = &tm.subject_map;
            let poms = tm.po_maps.clone();

            add_non_join_related_ops(
                &tm.identifier,
                &poms,
                sm_ref,
                &search_map,
                plan,
                &base_iri,
            )?;

            Ok::<(), RMLTranslationError>(())
        })?;

        Ok(plan)
    }
}

fn partition_pom_join_nonjoin(
    poms: Vec<PredicateObjectMap>,
) -> (Vec<PredicateObjectMap>, Vec<PredicateObjectMap>) {
    let (mut ptm_poms, mut no_ptm_poms): (Vec<_>, Vec<_>) =
        poms.into_iter().partition(|pom| pom.contains_ptm());

    for pom in ptm_poms.iter_mut() {
        let graph_maps = pom.graph_maps.clone();
        let (ptm_oms, no_ptm_oms): (Vec<_>, Vec<_>) = pom
            .object_maps
            .clone()
            .into_iter()
            .partition(|om| om.parent_tm.is_some());

        pom.object_maps = ptm_oms;
        if !no_ptm_oms.is_empty() {
            no_ptm_poms.push(PredicateObjectMap {
                predicate_maps: pom.predicate_maps.clone(),
                object_maps: no_ptm_oms,
                graph_maps,
            });
        }
    }

    (ptm_poms, no_ptm_poms)
}

fn add_non_join_related_ops(
    tm_iri: &str,
    no_join_poms: &[PredicateObjectMap],
    sm: &SubjectMap,
    search_map: &SearchMap,
    plan: &RcRefCellPlan<Processed>,
    base_iri: &Option<String>,
) -> Result<(), PlanError> {
    if no_join_poms.is_empty() & sm.classes.is_empty() {
        return Ok(());
    }

    let variable_map = &search_map.variable_map;
    let target_map = &search_map.target_map;
    let mut plan = plan.borrow_mut();

    let tm_infos = extract_tm_infos_from_sm_poms(sm, no_join_poms);
    let projection = ProjectionTranslator {
        tm_infos:       &tm_infos,
        join_condition: vec![],
        is_parent:      false,
    }
    .translate();
    let mut next_plan = plan.apply(&projection, "ProjectionOp")?;

    let tms = extract_tm_infos_from_sm_poms(sm, no_join_poms);

    let extend_translator = ExtendTranslator {
        tms,
        variable_map,
        base_iri: base_iri.clone(),
    };
    let extend_op = extend_translator.translate();
    next_plan = next_plan.apply(&extend_op, "ExtendOp")?;

    // Generate quad patterns and group them by the logical targets using the
    // informations from the different term maps (subject, predicate, object)
    let lt_quads_map = &generate_lt_quads_from_spo(sm, no_join_poms);
    let fragment_translator = FragmentTranslator { lt_quads_map };
    let fragmenter = fragment_translator.translate();

    // Add the fragmenter operator which fragments/broadcast the incoming
    // stream of mapping tuples to N streams of targets/serializer based
    // on N logical targets
    let mut lt_id_vec = vec![lt_quads_map.keys().next().unwrap().clone()];
    if let Some(fragmenter) = fragmenter {
        next_plan = next_plan.fragment(fragmenter.clone())?;
        lt_id_vec = fragmenter.to;
    }

    for lt_id in lt_id_vec {
        let target = target_map.get(&lt_id).unwrap();
        let serialize_format = &target.data_format;
        let quads = lt_quads_map.get(&lt_id).unwrap();

        let serializer_op = serializer::translate_serializer_op(
            quads,
            serialize_format,
            variable_map,
        );

        let _ = next_plan
            .serialize_with_fragment(serializer_op, &lt_id)?
            .sink(target)?;

        //let _ = extended_plan.fragment(fragmenter)?.serialize(serializer_op);
    }

    Ok(())
}

fn add_join_related_ops(
    tm: &TriplesMap,
    join_poms: &[PredicateObjectMap],
    sm: &SubjectMap,
    search_map: &SearchMap,
    plan: &RcRefCellPlan<Processed>,
    base_iri: &Option<String>,
) -> Result<(), PlanError> {
    // HashMap pairing the attribute with the function generated from
    // PTM's subject map

    let search_tm_plan_map = &search_map.tm_rccellplan_map;
    let variable_map = &search_map.variable_map;
    let lt_target_map = &search_map.target_map;

    for pom in join_poms {
        let pms = &pom.predicate_maps;
        let oms = &pom.object_maps;

        for om in oms.iter() {
            let ptm_iri = rcterm_to_string(om.parent_tm.as_ref().ok_or(
                PlanError::GenericError(format!(
                    "Parent triples map needs to be present in OM: {:#?}",
                    om
                )),
            )?);

            //Search for the plan associated with the parent triples map's IRI
            let (ptm, other_plan) = search_tm_plan_map.get(&ptm_iri).ok_or(
                PlanError::GenericError(format!(
                    "Parent triples map IRI is wrong: {}",
                    &ptm_iri
                )),
            )?;

            //Preparing plan for adding the join operator
            let ptm_variable = variable_map.get(&ptm.identifier).unwrap();
            let ptm_alias =
                format!("join_{}", &ptm_variable[ptm_variable.len() - 1..]);

            //Check for appropriate join type and add them to the plan
            let mut joined_plan: Plan<Processed>;
            let pom_with_joined_ptm = vec![PredicateObjectMap {
                predicate_maps: pms.clone(),
                object_maps:    [om.clone()].to_vec(),
                graph_maps:     pom.graph_maps.clone(),
            }];

            let join_cond_opt = om.join_condition.as_ref();
            if let Some(join_condition) = join_cond_opt {
                // Join condition exists so a θ-join operator will be added
                let mut aliased_plan =
                    join(Rc::clone(plan), Rc::clone(other_plan))?
                        .alias(&ptm_alias)?;

                let tm_infos =
                    &extract_tm_infos_from_sm_poms(sm, &pom_with_joined_ptm);

                let left_projection = ProjectionTranslator {
                    tm_infos,
                    join_condition: vec![join_condition],
                    is_parent: false,
                }
                .translate();
                aliased_plan = aliased_plan.apply_left_fragment(
                    left_projection,
                    Cow::Borrowed("LeftProjection"),
                    Cow::Borrowed(&ptm_alias),
                )?;

                let right_projection = ProjectionTranslator {
                    tm_infos:       &[&ptm.subject_map.get_term_map_info()],
                    join_condition: vec![join_condition],
                    is_parent:      true,
                }
                .translate();

                aliased_plan = aliased_plan.apply_right_fragment(
                    right_projection,
                    Cow::Borrowed("RightProjection"),
                    Cow::Borrowed(&ptm_alias),
                )?;

                let child_attributes = &join_condition.child_attributes;
                let parent_attributes = &join_condition.parent_attributes;

                joined_plan = aliased_plan
                    .where_by(child_attributes.clone())?
                    .compared_to(parent_attributes.clone())?;
            } else if tm.logical_source == ptm.logical_source {
                // Sources are the same, so a natural join operator will be added
                // instead.
                let mut aliased_plan =
                    join(Rc::clone(plan), Rc::clone(other_plan))?
                        .alias(&ptm_alias)?;
                joined_plan = aliased_plan.natural_join()?;
            } else {
                let mut aliased_plan =
                    join(Rc::clone(plan), Rc::clone(other_plan))?
                        .alias(&ptm_alias)?;
                joined_plan = aliased_plan.cross_join()?;
            }

            // Prefix the attributes in the subject map with the alias of the PTM
            let mut ptm_sm_info = ptm.subject_map.tm_info.clone();

            if join_cond_opt.is_some() {
                ptm_sm_info.prefix_attributes(&ptm_alias);
            }

            // Pair the ptm subject iri function with an extended attribute
            let (_, ptm_sub_function) =
                extract_extend_function_from_term_map_info(
                    variable_map,
                    &ptm_sm_info,
                    base_iri,
                );
            let om_extend_attr =
                variable_map.get(&om.tm_info.identifier).unwrap().clone();

            let mut extend_pairs = translate_extend_pairs(
                variable_map,
                sm,
                &pom_with_joined_ptm,
                base_iri,
            );

            extend_pairs.insert(om_extend_attr, ptm_sub_function);

            let extend_op = Operator::ExtendOp {
                config: Extend { extend_pairs },
            };
            let mut extended_plan = joined_plan.apply(&extend_op, "Extend")?;

            let lt_quads_map =
                generate_lt_quads_from_spo(sm, &pom_with_joined_ptm);

            for lt_id in lt_quads_map.keys() {
                let quads = lt_quads_map.get(lt_id).unwrap();
                let target = lt_target_map.get(lt_id).unwrap();
                let serializer_op = translate_serializer_op(
                    quads,
                    &target.data_format,
                    variable_map,
                );

                extended_plan.serialize(serializer_op)?.sink(target)?;
            }
            //.serialize(serializer_op)?;
            //.sink(file_target(count));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashSet;

    use parser::extractors::io::parse_file;
    use parser::extractors::triplesmap_extractor::extract_triples_maps;
    use parser::rml_model::term_map::{self, TermMapInfo};
    use sophia_api::ns::rdfs::Literal;
    use sophia_api::prelude::Iri;
    use sophia_api::term::{FromTerm, LanguageTag, Term};
    use sophia_term::{GenericLiteral, RcTerm};
    use util::{extract_tm_infos_from_poms, extract_tm_infos_from_tm};

    use super::*;
    use crate::import_test_mods;

    import_test_mods!(rml, parser);

    #[ignore]
    #[test]
    fn test_get_attributes_term_map_info() {
        let identifier = "tm_1".to_string();
        let template_term_map_info = TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type: term_map::TermMapType::Template,
            term_value: new_term_value("{id}{firstname}{lastname}".to_string()),
            term_type: None,
            fun_map_opt: None,
        };

        let attributes = template_term_map_info.get_attributes();
        let check = new_hash_set(["id", "firstname", "lastname"].into());

        assert_eq!(attributes, check);

        let reference_term_map_info = TermMapInfo {
            term_map_type: term_map::TermMapType::Reference,
            term_value: new_term_value("aReferenceValue".to_string()),
            ..template_term_map_info
        };

        let attributes = reference_term_map_info.get_attributes();
        let check = new_hash_set(["aReferenceValue"].into());
        assert_eq!(attributes, check);
    }

    #[ignore]
    #[test]
    fn test_projection_operator() -> ExtractorResult<()> {
        let graph = load_graph!("rml/sample_mapping.ttl").unwrap();
        let mut triples_map_vec = extract_triples_maps(&graph)?;
        assert_eq!(triples_map_vec.len(), 1);

        let triples_map = triples_map_vec.pop().unwrap();
        let projection_ops = ProjectionTranslator {
            tm_infos:       &extract_tm_infos_from_tm(&triples_map),
            join_condition: vec![],
            is_parent:      false,
        }
        .translate();

        let projection = match projection_ops.borrow() {
            Operator::ProjectOp { config: proj } => proj,
            _ => panic!("Parsed wrong! Operator should be projection"),
        };

        let check_attributes =
            new_hash_set(["stop", "id", "latitude", "longitude"].to_vec());

        assert_eq!(projection.projection_attributes, check_attributes);

        Ok(())
    }

    fn new_term_value(value: String) -> RcTerm {
        RcTerm::from_term(GenericLiteral::LanguageString(
            value,
            LanguageTag::new_unchecked("en".to_string()),
        ))
    }

    fn new_hash_set(v: Vec<&str>) -> HashSet<String> {
        v.into_iter().map(|st| st.to_string()).collect()
    }

    #[ignore]
    #[test]
    fn test_extend_operator() -> ExtractorResult<()> {
        let graph = load_graph!("rml/sample_mapping.ttl").unwrap();
        let mut triples_map_vec = extract_triples_maps(&graph)?;
        assert_eq!(triples_map_vec.len(), 1);
        let triples_map = triples_map_vec.pop().unwrap();

        let variable_map = &generate_variable_map(&Document {
            triples_maps:     triples_map_vec,
            default_base_iri: None,
        });
        let mut tms = vec![&triples_map.subject_map.tm_info];
        let tms_poms = extract_tm_infos_from_poms(&triples_map.po_maps);
        tms.extend(tms_poms);

        let extend_translator = ExtendTranslator {
            tms,
            variable_map,
            base_iri: None,
        };
        let extend_op = extend_translator.translate();
        println!("{:#?}", extend_op);
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_operator_translation() -> ExtractorResult<()> {
        let document = parse_file(test_case!("rml/sample_mapping.ttl").into())?;
        let operators =
            OptimizedRMLDocumentTranslator::translate_to_plan(document);

        let _output = File::create("op_trans_output.json").unwrap();
        println!("{:#?}", operators);
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_operator_translation_complex() -> ExtractorResult<()> {
        let document = parse_file(test_case!("rml/multiple_tm.ttl").into())?;
        let operators =
            OptimizedRMLDocumentTranslator::translate_to_plan(document);

        let _output = File::create("op_trans_complex_output.json").unwrap();
        println!("{:#?}", operators);
        Ok(())
    }
}
