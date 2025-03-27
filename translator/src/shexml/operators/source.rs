use std::collections::{HashMap, HashSet};

use log::{debug, trace};
use operator::formats::ReferenceFormulation;
use operator::{IOType, Source};
use plangenerator::error::PlanError;

use crate::shexml::error::ShExMLTranslationError;
use crate::shexml::parcombi::{
    self, ExpressionStmtEnum, IndexedShExMLDocument, Iterator, SourceType,
};
use crate::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct ShExMLSourceTranslator<'a> {
    pub document: &'a IndexedShExMLDocument,
}

pub type SourceExprIdentVecPair = (Source, Vec<String>);
pub type ShExMLSourceTranslatorOutput =
    Result<HashMap<String, SourceExprIdentVecPair>, ShExMLTranslationError>;

impl<'a> OperatorTranslator<ShExMLSourceTranslatorOutput>
    for ShExMLSourceTranslator<'a>
{
    fn translate(&self) -> ShExMLSourceTranslatorOutput {
        let ident_config_iotyperes_iter: Vec<_> = self
            .document
            .sources
            .values()
            .map(|source| {
                let mut config = HashMap::new();
                config.insert("path".to_string(), source.uri.clone());
                let source_type_res = match &source.source_type {
                    SourceType::File => Ok(IOType::File),
                    unsupported_type => {
                        Err(PlanError::GenericError(format!(
                            "Unsupported ShExML source type {:?}",
                            unsupported_type
                        )))
                    }
                };
                source_type_res.map(|source_type| {
                    (source.ident.as_str(), (config, source_type))
                })
            })
            .collect::<Result<_, PlanError>>()?;

        let ident_config_iotype_map: HashMap<_, _> =
            ident_config_iotyperes_iter.into_iter().collect();

        trace!(
            "Generated ident config iotype map: {:#?}",
            ident_config_iotype_map
        );

        debug!("Starting translation of shexml iterators to iterators for source operator.");
        debug!("Starting pairing of (source id, iterator id) with the associated (expression id)");
        let ident_iterators_map: &HashMap<_, _> = &self.document.iterators;

        let sourceid_iterid_pair_exprid: Vec<((&str, &str), &str)> = self
            .document
            .expression_stmts
            .values()
            .flat_map(|expr_stmt| {
                extract_source_iter_pairs(&expr_stmt.expr_enum)
                    .into_iter()
                    .map(|pair| (pair, expr_stmt.ident.as_str()))
            })
            .collect();
        trace!(
            "((Source id + iterator id), expr id) pairs:\n {:#?}",
            sourceid_iterid_pair_exprid
        );
        let mut source_expr_idents_map = HashMap::new();
        for ((source_ident, iter_ident), expr_ident) in
            sourceid_iterid_pair_exprid
        {
            let key = format!("{}.{}", source_ident, iter_ident);
            if let Some(source_exprs_pair) =
                source_expr_idents_map.get_mut(&key)
            {
                let (_, exprs): &mut (Source, Vec<String>) = source_exprs_pair;

                exprs.push(expr_ident.to_string());
            } else {
                let config_iotype_pair =
                    ident_config_iotype_map.get(source_ident).unwrap();

                let iter = ident_iterators_map.get(iter_ident).unwrap();
                let source = Source {
                    config:        config_iotype_pair.0.clone(),
                    source_type:   config_iotype_pair.1.clone(),
                    root_iterator: translate_to_operator_iterator(iter),
                };

                let value = (source, vec![expr_ident.to_string()]);

                source_expr_idents_map.insert(key, value);
            }
        }

        Ok(source_expr_idents_map)
    }
}

fn extract_source_iter_pairs(
    expr_enum: &ExpressionStmtEnum,
) -> HashSet<(&str, &str)> {
    let mut result = HashSet::new();
    match expr_enum {
        ExpressionStmtEnum::Union(left_box, right_box)
        | ExpressionStmtEnum::Join(left_box, right_box) => {
            let left_pairs = extract_source_iter_pairs(left_box);
            let right_pairs = extract_source_iter_pairs(right_box);

            result.extend(left_pairs);
            result.extend(right_pairs);
        }
        ExpressionStmtEnum::ConcatenateString {
            left_reference,
            right_reference,
            ..
        } => {
            result.insert((
                &left_reference.source_ident,
                &left_reference.iterator_ident,
            ));
            result.insert((
                &right_reference.source_ident,
                &right_reference.iterator_ident,
            ));
        }
        ExpressionStmtEnum::Basic { reference } => {
            result.insert((&reference.source_ident, &reference.iterator_ident));
        }
    };

    result
}

fn translate_to_operator_iterator(
    shexml_iter: &Iterator,
) -> operator::Iterator {
    let reference_formulation = translate_to_reference_formulation(
        shexml_iter.iter_type.as_ref().unwrap(),
    );

    let fields =
        translate_to_operator_fields(shexml_iter, &reference_formulation);

    operator::Iterator {
        alias: Some(shexml_iter.ident.to_string()),
        reference: shexml_iter.query.clone(),
        reference_formulation,
        fields,
    }
}

fn translate_to_reference_formulation(
    shex_iter_type: &parcombi::IteratorType,
) -> ReferenceFormulation {
    match shex_iter_type {
        parcombi::IteratorType::JSONPath => ReferenceFormulation::JSONPath,
        parcombi::IteratorType::XPath => ReferenceFormulation::XMLPath,
        parcombi::IteratorType::CSVRows => ReferenceFormulation::CSVRows,
        parcombi::IteratorType::SQL => ReferenceFormulation::SQLQuery,
        parcombi::IteratorType::SPARQL => ReferenceFormulation::SPARQL,
    }
}

fn translate_to_operator_fields(
    parent_shex_iter: &parcombi::Iterator,
    ref_formulation: &ReferenceFormulation,
) -> Vec<operator::Field> {
    let mut result = Vec::new();
    let flat_fields: Vec<operator::Field> = parent_shex_iter
        .fields
        .iter()
        .filter_map(|field| translate_to_flat_fields(field, ref_formulation))
        .collect();
    result.extend(flat_fields);

    let nested_iterator_fields: Vec<operator::Field> = parent_shex_iter
        .nested_iterator
        .iter()
        .map(|nested_iter| {
            operator::Field {
                alias:                 nested_iter.ident.clone(),
                reference:             nested_iter
                    .query
                    .clone()
                    .expect("Nested iterator needs a query string"),
                reference_formulation: ref_formulation.clone(),
                inner_fields:          translate_to_operator_fields(
                    nested_iter,
                    ref_formulation,
                ),
            }
        })
        .collect();

    result.extend(nested_iterator_fields);
    result
}

fn translate_to_flat_fields(
    shex_field: &parcombi::Field,
    ref_formulation: &ReferenceFormulation,
) -> Option<operator::Field> {
    match shex_field.field_type {
        parcombi::FieldType::Push | parcombi::FieldType::Normal => {
            Some(operator::Field {
                alias:                 shex_field.ident.clone(),
                reference:             shex_field.query.clone(),
                reference_formulation: ref_formulation.clone(),
                inner_fields:          vec![],
            })
        }
        parcombi::FieldType::Pop => None,
    }
}

#[cfg(test)]
mod tests {
    use parcombi::errors::{
        ParseCombiError, ParseCombiErrorKind, ShExMLParseCombiResult,
    };

    use super::*;
    use crate::test_case;

    #[test]
    fn source_translate_test() -> ShExMLParseCombiResult<()> {
        let simple_shexml = test_case!("shexml/straight_csv/input.shexml");
        let shexml_doc =
            parcombi::parse_file(simple_shexml)?.convert_to_indexed();
        let source_translator = ShExMLSourceTranslator {
            document: &shexml_doc,
        };

        let alge_source = source_translator.translate().map_err(|err| {
            ParseCombiError {
                dbg_msg: err.to_string(),
                msg:     err.to_string(),
                kind:    ParseCombiErrorKind::IOError,
            }
        })?;
        let expected_source_ids = vec!["films_csv_file.film_csv"];

        for expected_source_id in expected_source_ids {
            let (source, expr_ident) =
                alge_source.get(expected_source_id).unwrap_or_else(|| {
                    panic!(
                        "Expected {} source to be parsed \n\
                    But only these sources: {:#?}, got parsed! ",
                        expected_source_id,
                        alge_source.keys().collect::<Vec<_>>()
                    )
                });
            println!("Expr idents: {:#?}", expr_ident);
            println!("Source: {:#?}", source);
        }

        Ok(())
    }
}
