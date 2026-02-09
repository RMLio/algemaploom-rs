use std::collections::{HashMap, HashSet};

use log::{debug, trace};

use crate::shexml::parcombi::{
    GraphShapes, IndexedShExMLDocument, Object, Predicate, ShapeExpression,
    ShapeIdent, Subject,
};

#[derive(Debug, Clone)]
pub struct IndexVariableTerm<'a> {
    pub subject_variable_index: HashMap<&'a Subject, String>,
    pub object_variable_index:  HashMap<&'a Object, String>,
}

pub fn variablelize_quads<'a>(
    quads: &'a ShExMLQuads<'a>,
) -> IndexVariableTerm<'a> {
    let mut subject_variable_index = HashMap::new();
    let mut object_variable_index = HashMap::new();

    for (idx, (subj, _, obj, graph)) in quads.iter().enumerate() {
        if !subject_variable_index.contains_key(subj) {
            let subject_variable = format!("{}_sm_{}", graph.local, idx);
            subject_variable_index.insert(*subj, subject_variable);
        }

        if !object_variable_index.contains_key(obj) {
            let object_variable = format!("{}_om_{}", graph.local, idx);
            object_variable_index.insert(*obj, object_variable);
        }
    }

    IndexVariableTerm {
        subject_variable_index,
        object_variable_index,
    }
}

pub fn convert_graph_shape_to_quads(
    graph_shapes: &GraphShapes,
) -> Vec<(&Subject, &Predicate, &Object, &ShapeIdent)> {
    let graph_ident = &graph_shapes.ident;

    debug!("Converting graph shape to quads");
    let mut result = Vec::new();

    for shape in &graph_shapes.shapes {
        let quads = shape
            .pred_obj_pairs
            .iter()
            .map(|(pred, obj)| (&shape.subject, pred, obj, graph_ident));
        result.extend(quads);
    }

    trace!("Converted quads without filter: {:#?}", result);

    result
}

fn check_subj_expr_ident(subj: &Subject, expr_idents: &HashSet<&str>) -> bool {
    match &subj.expression {
        ShapeExpression::Reference(reference) => {
            expr_idents.contains(reference.expr_ident.as_str())
        }
        ShapeExpression::Matching {
            reference,
            matcher_ident: _,
        } => expr_idents.contains(reference.expr_ident.as_str()),
        ShapeExpression::Conditional {
            reference,
            conditional_expr: _,
        } => expr_idents.contains(reference.expr_ident.as_str()),
        ShapeExpression::Static { value: _ } => true,

        _ => false,
    }
}

fn check_obj_expr_ident(
    indexed_document: &IndexedShExMLDocument,
    obj: &Object,
    expr_idents: &HashSet<&str>,
    graph_ident: &ShapeIdent,
) -> bool {
    let obj_expr_bool = check_obj_related_expression(
        indexed_document,
        &obj.expression,
        expr_idents,
        graph_ident,
    );
    if let Some(language_expr) = &obj.language {
        obj_expr_bool
            && check_obj_related_expression(
                indexed_document,
                language_expr,
                expr_idents,
                graph_ident,
            )
    } else if let Some(datatype) = &obj.datatype {
        obj_expr_bool
            && check_obj_related_expression(
                indexed_document,
                &datatype.local_expr,
                expr_idents,
                graph_ident,
            )
    } else {
        obj_expr_bool
    }
}

fn check_obj_related_expression(
    indexed_document: &IndexedShExMLDocument,
    expression: &ShapeExpression,
    expr_idents: &HashSet<&str>,
    graph_ident: &ShapeIdent,
) -> bool {
    match expression {
        ShapeExpression::Reference(reference) => {
            expr_idents.contains(reference.expr_ident.as_str())
        }
        ShapeExpression::Matching {
            reference,
            matcher_ident: _,
        } => expr_idents.contains(reference.expr_ident.as_str()),
        ShapeExpression::Conditional {
            reference,
            conditional_expr: _,
        } => expr_idents.contains(reference.expr_ident.as_str()),
        ShapeExpression::Static { value: _ } => true,
        ShapeExpression::Link { other_shape_ident } => {
            debug!("Object is a shape link!");
            trace!("Other shape ident is: {}", other_shape_ident);
            let shape_map = &indexed_document.shapes;
            trace!("Shape map is: {:#?}", shape_map);
            let shape_opt = shape_map
                .get(&(graph_ident.clone(), other_shape_ident.clone()));
            if let Some(shape) = shape_opt {
                trace!("Other Shape map is: {:#?}", shape);
                check_subj_expr_ident(&shape.subject, expr_idents)
            } else {
                false
            }
        }
        shape_expr => {
            debug!("Unsupported shape expression for object to generate quads: {:?}", shape_expr);
            false
        }
    }
}

pub type ShExMLQuads<'a> =
    Vec<(&'a Subject, &'a Predicate, &'a Object, &'a ShapeIdent)>;

pub fn get_quads_from_same_source<'a>(
    indexed_document: &'a IndexedShExMLDocument,
    graph_shapes: impl std::iter::Iterator<Item = &'a GraphShapes>,
    expr_idents: HashSet<&'a str>,
) -> ShExMLQuads<'a> {
    get_quads_from_shapes(
        indexed_document,
        graph_shapes,
        expr_idents,
        |subj_check, obj_check| subj_check && obj_check,
    )
}

pub fn get_quads_from_different_source<'a>(
    indexed_document: &'a IndexedShExMLDocument,
    graph_shapes: impl std::iter::Iterator<Item = &'a GraphShapes>,
    expr_idents: HashSet<&'a str>,
) -> ShExMLQuads<'a> {
    get_quads_from_shapes(
        indexed_document,
        graph_shapes,
        expr_idents,
        |subj_check, obj_check| subj_check || obj_check,
    )
}

fn get_quads_from_shapes<'a, CheckerFn>(
    indexed_document: &'a IndexedShExMLDocument,
    graph_shapes: impl std::iter::Iterator<Item = &'a GraphShapes>,
    expr_idents: HashSet<&'a str>,
    source_checker: CheckerFn,
) -> ShExMLQuads<'a>
where
    CheckerFn: Fn(bool, bool) -> bool,
{
    let mut result = Vec::new();

    for graph in graph_shapes {
        let quads = convert_graph_shape_to_quads(graph);
        for quad in quads {
            let (subj, _, obj, graph_ident) = quad;
            let subj_is_in_source = check_subj_expr_ident(subj, &expr_idents);
            let obj_is_in_source = check_obj_expr_ident(
                indexed_document,
                obj,
                &expr_idents,
                graph_ident,
            );
            trace!("Subject check: {}", subj_is_in_source);
            trace!("Object check: {}", obj_is_in_source);

            if source_checker(subj_is_in_source, obj_is_in_source) {
                result.push(quad);
            }
        }
    }
    result
}
