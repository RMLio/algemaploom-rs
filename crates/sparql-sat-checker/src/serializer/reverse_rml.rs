use super::error::{PlanBuildError, PlanBuildResult};
use crate::{
    checker::util::{extract_extend_expr_from_operator, spo_from_trmap_sub_expression},
    extractor::rml::trmap_sub_expression::TrMapSubExpression
    ,
};
use operator::{
    formats::ReferenceFormulation, Field, Function, Operator,
};
use plan::data_type::DiGraphOperators;
use translator_normalized_rml::{OBJECT_ATTR, PREDICATE_ATTR, SUBJECT_ATTR};
use uuid::Uuid;
fn add_prefixes(buffer: &mut Vec<String>) {
    buffer.push("@base  <http://example.com/ns#>.".to_string());
    buffer.push(format!(
        "@prefix {}: <{}>.",
        vocab::rml::PREFIX,
        vocab::rml::IRI
    ));
    buffer.push(format!(
        "@prefix {}: <{}>.",
        vocab::r2rml::PREFIX,
        vocab::r2rml::IRI
    ));
    buffer.push(format!(
        "@prefix {}: <{}>.",
        vocab::query::PREFIX,
        vocab::query::IRI
    ));
}

fn extend_func_to_rml(
    func: &Function,
    fields: &[Field],
    no_rr_constant: bool,
) -> PlanBuildResult<String> {
    match func {
        Function::Nop => Ok("".to_string()),
        Function::Constant { value } => Ok(value.clone()),

        Function::Concatenate {
            left_value,
            separator,
            right_value,
        } => {
            let left_string = extend_func_to_rml(left_value, fields, true)?;
            let right_string = extend_func_to_rml(right_value, fields, true)?;
            Ok(format!("{}{}{}", left_string, separator, right_string))
        }
        Function::Reference { value } => {
            let res = retrieve_references_from_attributes(fields, value);
            if no_rr_constant {
                Ok(format!("{{{}}}", res))
            } else {
                Ok(res)
            }
        }
        Function::TypedConstant { value, term_type } => match term_type {
            operator::TermType::Literal => {
                if no_rr_constant {
                    Ok(value.to_string())
                } else {
                    Ok(format!("rr:constant \"{}\" ;", value))
                }
            }
            operator::TermType::IRI => {
                if no_rr_constant {
                    Ok(format!("<{}>", value))
                } else {
                    Ok(format!("rr:constant <{}> ;", value))
                }
            }
        },
        Function::UriEncode { inner_function } => extend_func_to_rml(inner_function, fields, false),
        Function::Iri {
            base_iri: _,
            inner_function,
        } => {
            let value = extend_func_to_rml(inner_function, fields, false)?;
            let value_term = value_term_string(inner_function, value)?;
            Ok(format!("rr:termType rr:IRI; \n {}", value_term))
        }
        Function::Literal {
            inner_function,
            dtype_function,
            langtype_function,
        } => {
            let value = extend_func_to_rml(inner_function, fields, false)?;
            let value_term = value_term_string(inner_function, value)?;
            let mut result = format!("rr:termType rr:Literal; \n {}", value_term);
            if let Some(dtype) = dtype_function.as_ref() {
                let dtype_value = extend_func_to_rml(dtype, fields, true)?;
                result = format!("{}\n rr:datatype {}", result, dtype_value);
            }
            if let Some(_lang_type) = langtype_function.as_ref() {
                unimplemented!("Language types not supported yet for now");
            }

            Ok(result)
        }
        Function::BlankNode { inner_function: _ } => todo!(),
        _ => Err(PlanBuildError::IncorrectInnerFunction(
            format!("{:?}", func),
            "unsupported!".to_string(),
        )),
    }
}

fn retrieve_references_from_attributes(fields: &[Field], value: &str) -> String {
    let iter = fields.iter().find(|iter| iter.alias == value);
    iter.unwrap().reference.clone().expect("Reference attribute in the given field is empty")
}

fn value_term_string(
    inner_function: &std::rc::Rc<Function>,
    value: String,
) -> PlanBuildResult<String> {
    match &**inner_function {
        Function::Concatenate {
            left_value: _,
            separator: _,
            right_value: _,
        } => Ok(format!("rr:template \"{}\";", value)),
        Function::Reference { value: _ } => Ok(format!("rml:reference \"{}\";", value)),
        _ => Err(PlanBuildError::IncorrectInnerFunction(
            format!("{:?}", inner_function),
            "inner function is not concatenate nor reference".to_string(),
        )),
    }
}
pub fn serialize_trmap_expr_to_rml(
    trmap_sub_exprs: &[TrMapSubExpression],
    graph: &DiGraphOperators,
) -> PlanBuildResult<String> {
    let mut buffer: Vec<String> = vec![];
    add_prefixes(&mut buffer);

    for trmap in trmap_sub_exprs {
        let mut triple_map_buffer = vec![];
        triple_map_buffer.push(format!("<{}> a rr:TriplesMap; ", Uuid::new_v4()));
        let operators: Vec<_> = trmap
            .node_indices
            .iter()
            .flat_map(|idx| graph.node_weight(*idx))
            .map(|node| &node.operator)
            .collect();

        let mut sources = operators.iter().filter_map(|op| match op {
            Operator::SourceOp { config } => Some(config),
            _ => None,
        });
        let (subject_node, predicate_node, object_node) =
            spo_from_trmap_sub_expression(graph, trmap);
        let (subject_op, predicate_op, object_op) = (
            &subject_node.operator,
            &predicate_node.operator,
            &object_node.operator,
        );
        let subject_extend_func = extract_extend_expr_from_operator(subject_op, SUBJECT_ATTR)?;
        let predicate_extend_func =
            extract_extend_expr_from_operator(predicate_op, PREDICATE_ATTR)?;
        let object_extend_func = extract_extend_expr_from_operator(object_op, OBJECT_ATTR)?;

        if let Some(join) = operators
            .iter()
            .filter_map(|op| match op {
                Operator::JoinOp { config } => Some(config),
                _ => None,
            })
            .next()
        {
            let source1 = sources.next().unwrap();
            let source2 = sources.next().unwrap();
            let ptm_iri = format!("<{}>", Uuid::new_v4());
            let (child_source, parent_source) =
                child_parent_source_from_join(source1, source2, join);
            let child_fields = &child_source.root_iterator.fields;
            let parent_fields = &parent_source.root_iterator.fields;
            add_logical_source_to_buffer(&mut triple_map_buffer, child_source);
            add_subject_map_to_buffer(
                &mut triple_map_buffer,
                subject_extend_func,
                child_fields,
                true,
            )?;

            triple_map_buffer.push("rr:predicateObjectMap [".to_string());
            add_predicate_map_to_buffer(
                &mut triple_map_buffer,
                predicate_extend_func,
                child_fields,
            )?;
            triple_map_buffer.push("rr:objectMap [".to_string());
            triple_map_buffer.push(format!("rr:parentTriplesMap {};", ptm_iri));
            for (child, parent) in &join.left_right_attr_pairs {
                triple_map_buffer.push("rr:joinCondition [".to_string());
                let child = retrieve_references_from_attributes(child_fields, child);
                triple_map_buffer.push(format!("rr:child \"{}\";", child));
                let parent = retrieve_references_from_attributes(parent_fields, parent);
                triple_map_buffer.push(format!("rr:parent \"{}\";", parent));
                triple_map_buffer.push("];".to_string());
            }
            triple_map_buffer.push("];".to_string());
            triple_map_buffer.push("].".to_string());

            // Parent triples map
            let mut parent_triple_map_buffer = vec![];
            parent_triple_map_buffer.push(format!("{} a rr:TriplesMap; ", ptm_iri));
            add_logical_source_to_buffer(&mut parent_triple_map_buffer, parent_source);
            add_subject_map_to_buffer(
                &mut parent_triple_map_buffer,
                object_extend_func,
                parent_fields,
                false,
            )?;
            buffer.extend(parent_triple_map_buffer);
        } else {
            let source = sources.next().unwrap();
            add_logical_source_to_buffer(&mut triple_map_buffer, source);

            let fields = &source.root_iterator.fields;
            add_subject_map_to_buffer(&mut triple_map_buffer, subject_extend_func, fields, true)?;
            triple_map_buffer.push("rr:predicateObjectMap [".to_string());
            add_predicate_map_to_buffer(&mut triple_map_buffer, predicate_extend_func, fields)?;
            triple_map_buffer.push("rr:objectMap [".to_string());
            let object_value = extend_func_to_rml(object_extend_func, fields, false)?;
            triple_map_buffer.push(object_value);
            triple_map_buffer.push("];".to_string());
            triple_map_buffer.push("].".to_string());
        }

        buffer.extend(triple_map_buffer);
    }
    Ok(buffer.join("\n"))
}

fn add_predicate_map_to_buffer(
    triple_map_buffer: &mut Vec<String>,
    predicate_extend_func: &Function,
    child_fields: &Vec<Field>,
) -> Result<(), PlanBuildError> {
    triple_map_buffer.push("rr:predicateMap [".to_string());
    let predicate_term_map = extend_func_to_rml(predicate_extend_func, child_fields, false)?;
    triple_map_buffer.push(predicate_term_map);
    triple_map_buffer.push("];".to_string());
    Ok(())
}

fn add_subject_map_to_buffer(
    triple_map_buffer: &mut Vec<String>,
    subject_extend_func: &Function,
    fields: &[Field],
    is_not_final: bool,
) -> Result<(), PlanBuildError> {
    let subject_term_map = extend_func_to_rml(subject_extend_func, fields, false)?;
    triple_map_buffer.push("rr:subjectMap [".to_string());
    triple_map_buffer.push(subject_term_map);
    if is_not_final {
        triple_map_buffer.push("];".to_string());
    } else {
        triple_map_buffer.push("].".to_string());
    }
    Ok(())
}

fn add_logical_source_to_buffer(buffer: &mut Vec<String>, source: &operator::Source) {
    let root_iter = &source.root_iterator;
    buffer.push("rml:logicalSource [ ".to_string());
    //Only works for files for now
    buffer.push(format!(
        "rml:source \"{}\"; ",
        source.config.get("path").unwrap()
    ));
    buffer.push(format!(
        "rml:referenceFormulation {}; ",
        refform_to_iri(&root_iter.reference_formulation)
    ));
    if let Some(root_ref) = root_iter.reference.as_ref() {
        buffer.push(format!("rml:iterator \"{}\"; ", root_ref));
    }
    buffer.push("];".to_string());
}
fn refform_to_iri(ref_form: &ReferenceFormulation) -> String {
    match ref_form {
        ReferenceFormulation::CSVRows => "ql:CSV".to_string(),
        ReferenceFormulation::JSONPath => "ql:JSONPath".to_string(),
        ReferenceFormulation::XMLPath | ReferenceFormulation::XMLQuery => "ql:XPath".to_string(),
        ReferenceFormulation::SQLQuery => "ql:SQL".to_string(),
        ReferenceFormulation::SPARQL => "ql:SPARQL".to_string(),
        ReferenceFormulation::CSS3 => "ql:CSS3".to_string(),
    }
}

fn child_parent_source_from_join<'a>(
    source1: &'a operator::Source,
    source2: &'a operator::Source,
    join: &'a operator::Join,
) -> (&'a operator::Source, &'a operator::Source) {
    let (child, _) = join.left_right_attr_pairs.first().unwrap();
    if source1
        .root_iterator
        .fields
        .iter()
        .map(|f| &f.alias)
        .any(|f| f == child)
    {
        (source1, source2)
    } else {
        (source2, source1)
    }
}
