use std::collections::{HashMap, HashSet};

use log::{debug, trace};
use shexml_interpreter::{
    ExpressionReferenceIdent, ExpressionStmt, ExpressionStmtEnum, FieldType,
    Iterator,
};

fn update_rename_map_iterator(
    parent: &str,
    to_prefix: &str,
    iterator: &Iterator,
    rename_pairs: &mut HashMap<String, String>,
) {
    let normal_fields = iterator
        .fields
        .iter()
        .filter(|field| field.field_type == FieldType::Normal);

    let from_prefix = match parent.is_empty() {
        true => iterator.ident.clone(),
        false => format!("{}.{}", parent, iterator.ident),
    };

    normal_fields.for_each(|field| {
        let from = format!("{}.{}", from_prefix, field.ident);
        let to = format!("{}.{}", to_prefix, field.ident);
        trace!("Updating rename pairs map with: {} -> {}", from, to);
        rename_pairs.insert(from, to);
    });

    for nested_iter in iterator.nested_iterator.iter() {
        let next_prefix = format!("{}.{}", to_prefix, nested_iter.ident);
        trace!("Prefix for the next nested iterator: {}", next_prefix);
        update_rename_map_iterator(
            &from_prefix,
            &next_prefix,
            nested_iter,
            rename_pairs,
        );
    }
}

pub fn translate_rename_pairs_map(
    iterators_map: &HashMap<String, Iterator>,
    expr_stmt: &ExpressionStmt,
    source_iter_ident: &str,
) -> HashMap<String, String> {
    let mut rename_pairs = HashMap::new();
    debug!("Translating rename pair maps for expression statement");
    trace!("Expression statement is: {:#?}", expr_stmt);
    let references = extract_reference_expr_enum(&expr_stmt.expr_enum)
        .into_iter()
        .filter(|reference| {
            format!("{}.{}", reference.source_ident, reference.iterator_ident)
                == source_iter_ident
        });

    for reference in references {
        let iter_ident = &reference.iterator_ident;
        let expr_ident = &expr_stmt.ident;

        if let Some(field) = &reference.field {
            let from = format!("{}.{}", iter_ident, field);
            let to = expr_ident.to_string();

            rename_pairs.insert(from, to);
        } else if let Some(iterator) = iterators_map.get(iter_ident) {
            debug!("Expression statement doesn't reference iterator's field directly");
            trace!(
                "Updating rename map using iterator's fields implicitly: {:#?}",
                iterator
            );
            update_rename_map_iterator(
                "",
                expr_ident,
                iterator,
                &mut rename_pairs,
            );
        }
    }
    rename_pairs
}

fn extract_reference_expr_enum(
    expr_enum: &ExpressionStmtEnum,
) -> HashSet<ExpressionReferenceIdent> {
    match expr_enum {
        ExpressionStmtEnum::Join(left_enum, right_enum)
        | ExpressionStmtEnum::Union(left_enum, right_enum) => {
            let mut left_references = extract_reference_expr_enum(left_enum);
            let right_references = extract_reference_expr_enum(right_enum);

            left_references.extend(right_references);
            left_references
        }
        ExpressionStmtEnum::Basic { reference } => {
            HashSet::from([reference.clone()])
        }
        _ => HashSet::new(),
    }
}
