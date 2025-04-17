use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use either::Either;
use log::trace;
use operator::Function;

use crate::shexml::parcombi::{
    ExpressionReferenceIdent, ExpressionStmtEnum, Iterator
};

pub mod term;

pub fn translate_concatenate_to_extend_pairs(
    expr_ident: &str,
    iterators: &HashMap<String, Iterator>,
    expr_enum: &ExpressionStmtEnum,
    source_iter_ident: &str,
) -> Vec<(String, Function)> {
    let mut result = Vec::new();

    match expr_enum {
        ExpressionStmtEnum::Union(
            left_enum,
            right_enum,
        ) => {
            result.extend(translate_concatenate_to_extend_pairs(
                expr_ident,
                iterators,
                left_enum,
                source_iter_ident,
            ));

            result.extend(translate_concatenate_to_extend_pairs(
                expr_ident,
                iterators,
                right_enum,
                source_iter_ident,
            ));
        }
        ExpressionStmtEnum::ConcatenateString {
            left_reference,
            concate_string,
            right_reference,
        } => {
            let left_source_iter_ident = format!(
                "{}.{}",
                left_reference.source_ident, left_reference.iterator_ident
            );
            let right_source_iter_ident = format!(
                "{}.{}",
                right_reference.source_ident, right_reference.iterator_ident
            );

            if left_source_iter_ident == source_iter_ident
                && right_source_iter_ident == source_iter_ident
            {
                result.extend(generate_extend_concate_pair(
                    iterators,
                    left_reference,
                    right_reference,
                    expr_ident,
                    concate_string,
                ));
            }
        }
        _ => {
            trace!("Concatenate extraction doesn't support the expression of type: {:?}", expr_enum);
        }
    };

    result
}

fn generate_extend_concate_pair(
    iterators: &HashMap<String, Iterator>,
    left_reference: &ExpressionReferenceIdent,
    right_reference: &ExpressionReferenceIdent,
    expr_ident: &str,
    concate_string: &str,
) -> Vec<(String, Function)> {
    let mut result = Vec::new();
    let either_leftside = get_fields(iterators, left_reference);
    let either_rightside = get_fields(iterators, right_reference);
    if let Either::Left(left_field) = either_leftside {
        let right_field = either_rightside
                .expect_left(
                    &format!("Right reference of string op, {}, needs to specify a single field.", expr_ident)
                    );

        let func =
            create_concat_function(left_field, right_field, concate_string);
        result.push((expr_ident.to_string(), func));
    } else if let Either::Right(left_fields) = either_leftside {
        let right_fields = either_rightside.expect_right(&format!(
            "Right reference of string op, {}, needs to end at the iterator",
            expr_ident
        ));

        let intersected_fields = left_fields.intersection(&right_fields);

        for field in intersected_fields {
            let left_field =
                format!("{}.{}", left_reference.iterator_ident, field);
            let right_field =
                format!("{}.{}", right_reference.iterator_ident, field);

            let func =
                create_concat_function(left_field, right_field, concate_string);

            result.push((format!("{}.{}", expr_ident, field), func))
        }
    }

    result
}

fn get_fields(
    iterators: &HashMap<String, Iterator>,
    expr_ref_ident: &ExpressionReferenceIdent,
) -> Either<String, HashSet<String>> {
    if let Some(field) = &expr_ref_ident.field {
        Either::Left(format!("{}.{}", expr_ref_ident.iterator_ident, field))
    } else {
        let iter_ident = &expr_ref_ident.iterator_ident;
        let iter = iterators.get(iter_ident).unwrap();
        Either::Right(
            iter.fields
                .iter()
                .map(|field| field.ident.clone())
                .collect(),
        )
    }
}

fn create_concat_function(
    left_field: String,
    right_field: String,
    concate_string: &str,
) -> Function {
    let left_value = Rc::new(Function::Reference { value: left_field });

    let right_value = Rc::new(Function::Reference { value: right_field });

    Function::Concatenate {
        left_value,
        separator: concate_string.to_string(),
        right_value,
    }
}
