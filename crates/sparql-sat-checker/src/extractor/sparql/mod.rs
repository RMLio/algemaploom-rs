use std::{fs::File, io::Read, path::Path};

use error::{SparqlExtractError, ExtractErrorKind, FromFileError, FromFileErrorKind};
use spargebra::{
    SparqlParser,
    algebra::{self, AggregateExpression, GraphPattern, PropertyPathExpression},
    term::{NamedNode, TriplePattern},
};
pub mod error; 

fn get_gp_from_query(query: spargebra::Query) -> GraphPattern {
    match query {
        spargebra::Query::Select {
            dataset: _,
            pattern,
            base_iri: _,
        } => pattern,
        spargebra::Query::Construct {
            template: _,
            dataset: _,
            pattern,
            base_iri: _,
        } => pattern,
        spargebra::Query::Describe {
            dataset: _,
            pattern,
            base_iri: _,
        } => pattern,
        spargebra::Query::Ask {
            dataset: _,
            pattern,
            base_iri: _,
        } => pattern,
    }
}

fn get_tps_from_gp(graph_pattern: algebra::GraphPattern) -> Vec<TriplePattern> {
    match graph_pattern {
        GraphPattern::Bgp { patterns } => patterns,
        GraphPattern::LeftJoin {
            left,
            right,
            expression,
        } => {
            let mut acc = get_tps_from_gp(*left);
            let right = get_tps_from_gp(*right);
            acc.extend(right);

            if let Some(expr) = expression {
                let expr = get_tps_from_expression(expr);
                acc.extend(expr);
            }
            acc
        }
        GraphPattern::Filter { expr, inner } => {
            let mut acc = get_tps_from_expression(expr);
            let inner = get_tps_from_gp(*inner);
            acc.extend(inner);
            acc
        }
        GraphPattern::Graph { name: _, inner } => get_tps_from_gp(*inner),
        GraphPattern::Extend {
            inner,
            variable: _,
            expression,
        } => {
            let mut acc = get_tps_from_gp(*inner);
            let expr = get_tps_from_expression(expression);
            acc.extend(expr);
            acc
        }
        GraphPattern::Join { left, right }
        | GraphPattern::Union { left, right }
        | GraphPattern::Minus { left, right } => {
            let mut acc = get_tps_from_gp(*left);
            let right = get_tps_from_gp(*right);
            acc.extend(right);
            acc
        }

        GraphPattern::OrderBy {
            inner,
            expression: _,
        }
        | GraphPattern::Project {
            inner,
            variables: _,
        }
        | GraphPattern::Distinct { inner }
        | GraphPattern::Reduced { inner }
        | GraphPattern::Slice {
            inner,
            start: _,
            length: _,
        }
        | GraphPattern::Service {
            name: _,
            inner,
            silent: _,
        } => get_tps_from_gp(*inner),
        GraphPattern::Group {
            inner,
            variables: _,
            aggregates,
        } => {
            let mut acc = get_tps_from_gp(*inner);
            let aggr = aggregates
                .into_iter()
                .flat_map(|(_, aggr)| get_tps_from_aggr_expr(aggr));
            acc.extend(aggr);
            acc
        }
        GraphPattern::Path {
            subject,
            path,
            object,
        } => {
            let nodes = get_nodes_from_property_path(path);
            nodes
                .into_iter()
                .map(|predicate| TriplePattern {
                    subject: subject.clone(),
                    predicate: predicate.into(),
                    object: object.clone(),
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn get_nodes_from_property_path(path: PropertyPathExpression) -> Vec<NamedNode> {
    match path {
        PropertyPathExpression::NamedNode(named_node) => vec![named_node],
        PropertyPathExpression::NegatedPropertySet(named_nodes) => named_nodes,
        PropertyPathExpression::Sequence(property_path_expression, property_path_expression1)
        | PropertyPathExpression::Alternative(
            property_path_expression,
            property_path_expression1,
        ) => {
            let mut acc = get_nodes_from_property_path(*property_path_expression);
            let right = get_nodes_from_property_path(*property_path_expression1);
            acc.extend(right);
            acc
        }
        PropertyPathExpression::ZeroOrOne(property_path_expression)
        | PropertyPathExpression::OneOrMore(property_path_expression)
        | PropertyPathExpression::Reverse(property_path_expression)
        | PropertyPathExpression::ZeroOrMore(property_path_expression) => {
            get_nodes_from_property_path(*property_path_expression)
        }
    }
}

fn get_tps_from_aggr_expr(aggr: AggregateExpression) -> Vec<TriplePattern> {
    match aggr {
        AggregateExpression::FunctionCall {
            name: _,
            expr,
            distinct: _,
        } => get_tps_from_expression(expr),
        _ => Vec::new(),
    }
}

fn get_tps_from_expression(expr: algebra::Expression) -> Vec<TriplePattern> {
    match expr {
        algebra::Expression::Exists(graph_pattern) => get_tps_from_gp(*graph_pattern),
        algebra::Expression::FunctionCall(_function, expressions) => expressions
            .into_iter()
            .flat_map(get_tps_from_expression)
            .collect(),
        algebra::Expression::UnaryPlus(expression)
        | algebra::Expression::UnaryMinus(expression)
        | algebra::Expression::Not(expression) => get_tps_from_expression(*expression),
        algebra::Expression::Coalesce(expressions) => expressions
            .into_iter()
            .flat_map(get_tps_from_expression)
            .collect(),
        algebra::Expression::Or(expression, expression1)
        | algebra::Expression::And(expression, expression1)
        | algebra::Expression::Equal(expression, expression1)
        | algebra::Expression::SameTerm(expression, expression1)
        | algebra::Expression::Greater(expression, expression1)
        | algebra::Expression::GreaterOrEqual(expression, expression1)
        | algebra::Expression::Less(expression, expression1)
        | algebra::Expression::LessOrEqual(expression, expression1)
        | algebra::Expression::Add(expression, expression1)
        | algebra::Expression::Subtract(expression, expression1)
        | algebra::Expression::Multiply(expression, expression1)
        | algebra::Expression::Divide(expression, expression1) => {
            let mut acc = get_tps_from_expression(*expression);
            let right = get_tps_from_expression(*expression1);
            acc.extend(right);
            acc
        }
        algebra::Expression::In(expression, expressions) => {
            let mut acc = get_tps_from_expression(*expression);
            acc.extend(expressions.into_iter().flat_map(get_tps_from_expression));
            acc
        }
        algebra::Expression::If(expression, expression1, expression2) => {
            let mut acc = get_tps_from_expression(*expression);
            let expr1 = get_tps_from_expression(*expression1);
            let expr2 = get_tps_from_expression(*expression2);

            acc.extend(expr1);
            acc.extend(expr2);
            acc
        }
        _ => Vec::new(),
    }
}

pub fn extract_triple_patterns_from_sparql_str(sparql_str: &str) -> Result<Vec<TriplePattern>, SparqlExtractError> {
    let query = SparqlParser::new()
        .parse_query(sparql_str)
        .map_err(ExtractErrorKind::SparqlParseError)
        .map_err(|kind| SparqlExtractError {
            query: sparql_str.into(),
            kind,
        })?;
    let graph_pattern = get_gp_from_query(query);
    Ok(get_tps_from_gp(graph_pattern))
}

pub fn extract_triple_patterns_from_sparql_file(path: &Path) -> Result<Vec<TriplePattern>, FromFileError> {
    let mut file = File::open(path)
        .map_err(FromFileErrorKind::StdIoError)
        .map_err(|kind| FromFileError {
            path: path.into(),
            kind,
        })?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .map_err(FromFileErrorKind::StdIoError)
        .map_err(|kind| FromFileError {
            path: path.into(),
            kind,
        })?;

    extract_triple_patterns_from_sparql_str(&buffer)
        .map_err(FromFileErrorKind::ExtractError)
        .map_err(|kind| FromFileError {
            path: path.into(),
            kind,
        })
}
