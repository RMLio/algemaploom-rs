pub mod data;
mod extend_creation;
mod source_creation;
mod util;

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use extend_creation::create_extend_function;
use operator::formats::DataFormat;
use operator::{Extend, Function, Operator, Projection, Serializer, Target, TermType};
use oxigraph::model::SubjectRef;
use oxigraph::store::Store;
use plangenerator::states::join::join;
use plangenerator::states::{Init, Processed, Sunk};
use plangenerator::Plan;
use source_creation::create_source_operator;
use util::{get_object, get_quads, termref_to_literal, termref_to_subjref};
use vocab::ToString;

use crate::normalized_rml::{FromVocab, GRAPH_ATTR, OBJECT_ATTR, PREDICATE_ATTR, SUBJECT_ATTR};

pub fn translate_normalized_rml(store: &Store, base_iri: Option<String>) -> Result<Plan<Sunk>> {
    let mut plan = Plan::new();

    let mut previous_plan_opt: Option<Plan<Processed>> = None;
    for triples_map_quad in store
        .quads_for_pattern(
            None,
            Some((&vocab::rdf::PROPERTY::TYPE.to_named_node()).into()),
            Some((&vocab::r2rml::CLASS::TRIPLESMAP.to_term()).into()),
            None,
        )
        .filter_map(|res| res.ok())
    {
        let triples_map_iri = triples_map_quad.subject.as_ref();

        //Predicate object map's presence checked first before mutating the plan
        let predicate_object_map_res = get_object(
            triples_map_iri,
            vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP
                .to_named_node()
                .as_ref(),
            store,
        );
        // There can be triples maps without predicate object maps (still a valid RML graph)
        if predicate_object_map_res.is_err() {
            continue;
        }

        let (source, query_attr_map) = create_source_operator(triples_map_iri, store)?;

        let mut sourced_plan = plan.source(source);

        let subject = get_object(
            triples_map_iri,
            vocab::r2rml::PROPERTY::SUBJECTMAP.to_named_node().as_ref(),
            store,
        )?;
        let subject_ref = termref_to_subjref(subject.as_ref())?;
        let subject_function =
            create_extend_function(subject_ref, store, &query_attr_map, false, &base_iri)?;
        let subj_extend = operator::Operator::ExtendOp {
            config: Extend {
                extend_pairs: HashMap::from([(SUBJECT_ATTR.to_string(), subject_function)]),
            },
        };
        let mut processed_plan = sourced_plan.apply(&subj_extend, "Subject_Extend_").unwrap();

        let predicate_object_map = predicate_object_map_res?;
        let predicate_object_map_subjref = termref_to_subjref(predicate_object_map.as_ref())?;

        let predicate = get_object(
            predicate_object_map_subjref,
            vocab::r2rml::PROPERTY::PREDICATEMAP
                .to_named_node()
                .as_ref(),
            store,
        )?;
        let predicate_function = create_extend_function(
            termref_to_subjref(predicate.as_ref())?,
            store,
            &query_attr_map,
            false,
            &base_iri,
        )?;
        let pred_extend = Operator::ExtendOp {
            config: Extend {
                extend_pairs: HashMap::from([(PREDICATE_ATTR.to_string(), predicate_function)]),
            },
        };

        processed_plan = processed_plan
            .apply(&pred_extend, "Predicate_Extend_")
            .unwrap();

        processed_plan = process_object_map(
            store,
            &mut plan,
            processed_plan,
            predicate_object_map_subjref,
            &query_attr_map,
            &base_iri
        )?;

        let sm_gm_res = get_object(
            subject_ref,
            vocab::r2rml::PROPERTY::GRAPHMAP.to_named_node().as_ref(),
            store,
        );
        let pom_gm_res = get_object(
            predicate_object_map_subjref,
            vocab::r2rml::PROPERTY::GRAPHMAP.to_named_node().as_ref(),
            store,
        );

        let gm_res = sm_gm_res.or(pom_gm_res);
        let extend_func = if let Ok(gm) = gm_res {
            create_extend_function(
                termref_to_subjref(gm.as_ref())?,
                store,
                &query_attr_map,
                false,
                &base_iri,
            )?
        } else {
            Function::TypedConstant {
                value: vocab::r2rml::CLASS::DEFAULTGRAPH.to_string(),
                term_type: TermType::IRI,
            }
        };

        let extend_op = Operator::ExtendOp {
            config: Extend {
                extend_pairs: HashMap::from([(GRAPH_ATTR.to_string(), extend_func)]),
            },
        };

        processed_plan = processed_plan.apply(&extend_op, "Graph_Extend_").unwrap();
        if let Some(mut previous_plan) = previous_plan_opt {
            let proj_operator = Operator::ProjectOp {
                config: Projection {
                    projection_attributes: HashSet::from([
                        SUBJECT_ATTR.to_string(),
                        PREDICATE_ATTR.to_string(),
                        OBJECT_ATTR.to_string(),
                        GRAPH_ATTR.to_string(),
                    ]),
                },
            };
            processed_plan = processed_plan.apply(&proj_operator, "Projection_").unwrap();

            previous_plan_opt = Some(previous_plan.union(processed_plan.into()).unwrap());
        } else {
            previous_plan_opt = Some(processed_plan);
        }
    }

    let mut consolidated_plan = previous_plan_opt.unwrap();
    let template = format!(
        "?{} ?{} ?{} ?{} .",
        SUBJECT_ATTR, PREDICATE_ATTR, OBJECT_ATTR, GRAPH_ATTR
    );
    let serializer = Serializer {
        template,
        options: None,
        format: DataFormat::NQuads,
    };

    let sink = Target {
        configuration: HashMap::new(),
        target_type: operator::IOType::StdOut,
        data_format: DataFormat::NQuads,
    };

    let serialized_plan = consolidated_plan
        .serialize(serializer)
        .unwrap()
        .sink(&sink)
        .unwrap();

    Ok(serialized_plan)
}

fn process_object_map(
    store: &Store,
    main_plan: &mut Plan<Init>,
    mut processed_plan: Plan<Processed>,
    predicate_object_map_subjref: SubjectRef,
    child_query_attr_map: &HashMap<String, String>,
    base_iri: &Option<String>,
) -> Result<Plan<Processed>, anyhow::Error> {
    let object = get_object(
        predicate_object_map_subjref,
        vocab::r2rml::PROPERTY::OBJECTMAP.to_named_node().as_ref(),
        store,
    )?;
    let object_subjref = termref_to_subjref(object.as_ref())?;
    if let Ok(ptm) = get_object(
        object_subjref,
        vocab::r2rml::PROPERTY::PARENTTRIPLESMAP
            .to_named_node()
            .as_ref(),
        store,
    ) {
        let (source, parent_query_attr_map) =
            create_source_operator(termref_to_subjref(ptm.as_ref())?, store)?;
        let join_condition_pairs = get_join_condition_pairs(
            store,
            child_query_attr_map,
            &parent_query_attr_map,
            object_subjref,
        )?;

        let sourced_plan = main_plan.source(source);

        let mut joined_plan = join(processed_plan.into(), sourced_plan.into())
            .unwrap()
            .alias("")
            .unwrap()
            .where_by(join_condition_pairs.iter().map(|p| p.0).collect())
            .unwrap()
            .compared_to(join_condition_pairs.iter().map(|p| p.1).collect())
            .unwrap();

        let ptm_subj_ref = termref_to_subjref(ptm.as_ref())?;
        let subject_ptm = get_object(
            ptm_subj_ref,
            vocab::r2rml::PROPERTY::SUBJECTMAP.to_named_node().as_ref(),
            store,
        )?;
        let extend_func = create_extend_function(
            termref_to_subjref(subject_ptm.as_ref())?,
            store,
            &parent_query_attr_map,
            false,
            base_iri
        )?;

        let extend_operator = Operator::ExtendOp {
            config: Extend {
                extend_pairs: HashMap::from([(OBJECT_ATTR.to_string(), extend_func)]),
            },
        };
        Ok(joined_plan
            .apply(&extend_operator, "Object_Extend_")
            .unwrap())
    } else {
        let extend_func =
            create_extend_function(object_subjref, store, child_query_attr_map, true, base_iri)?;
        let extend_operator = Operator::ExtendOp {
            config: Extend {
                extend_pairs: HashMap::from([(OBJECT_ATTR.to_string(), extend_func)]),
            },
        };
        Ok(processed_plan
            .apply(&extend_operator, "Object_Extend_")
            .unwrap())
    }
}

fn get_join_condition_pairs<'a>(
    store: &'a Store,
    child_query_attr_map: &'a HashMap<String, String>,
    parent_query_attr_map: &'a HashMap<String, String>,
    object_subjref: oxigraph::model::SubjectRef<'a>,
) -> Result<Vec<(&'a str, &'a str)>, anyhow::Error> {
    let mut join_condition_pairs = vec![];
    for jc in get_quads(
        object_subjref,
        vocab::r2rml::PROPERTY::JOINCONDITION
            .to_named_node()
            .as_ref(),
        store,
    )
    .map(|q| q.object)
    {
        let jc_subj_ref = termref_to_subjref(jc.as_ref())?;

        let parent_term = get_object(
            jc_subj_ref,
            vocab::r2rml::PROPERTY::PARENT.to_named_node().as_ref(),
            store,
        )?;
        let parent = termref_to_literal(parent_term.as_ref())?;

        let child_term = get_object(
            jc_subj_ref,
            vocab::r2rml::PROPERTY::CHILD.to_named_node().as_ref(),
            store,
        )?;
        let child = termref_to_literal(child_term.as_ref())?;

        let parent_attr = parent_query_attr_map.get(parent.value()).unwrap();
        let child_attr = child_query_attr_map.get(child.value()).unwrap();

        join_condition_pairs.push((child_attr.as_ref(), parent_attr.as_ref()));
    }

    Ok(join_condition_pairs)
}
