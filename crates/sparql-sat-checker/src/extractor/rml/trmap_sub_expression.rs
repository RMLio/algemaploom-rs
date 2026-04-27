use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use operator::{display::PrettyDisplay, Operator};
use petgraph::{
    graph::NodeIndex,
    visit::{Dfs, Reversed},
};
use plan::data_type::DiGraphOperators;
use translator_normalized_rml::{OBJECT_ATTR, PREDICATE_ATTR, SUBJECT_ATTR};

use super::error::{RMLExtractorError, RMLResult};

#[derive(Debug, Clone, Default)]
pub struct TrMapSubExpression {
    //Sorted node indices of length n such that the
    //index of the "source" operator is at index n-1
    pub node_indices: Vec<NodeIndex>,
    pub subject_index: NodeIndex,
    pub predicate_index: NodeIndex,
    pub object_index: NodeIndex,
}

impl TrMapSubExpression {
    pub fn try_new(graph: &Reversed<&DiGraphOperators>, node_idx: NodeIndex) -> RMLResult<Self> {
        let mut dfs = Dfs::new(graph, node_idx);
        let mut res = Self::default();
        let mut attribute_node_map = HashMap::new();
        while let Some(idx) = dfs.next(graph) {
            res.add_node(idx, graph.0, &mut attribute_node_map);
        }

        if attribute_node_map.len() != 4 {
            return Err(RMLExtractorError::SpecialAttributesIncorrectAmount(
                format!(
                    "while extracting TrMapsSubExpression only found attributes: {:?}",
                    attribute_node_map.keys().collect::<Vec<_>>()
                ),
            ));
        } else {
            res.subject_index = *attribute_node_map.get(SUBJECT_ATTR).unwrap();
            res.predicate_index = *attribute_node_map.get(PREDICATE_ATTR).unwrap();
            res.object_index = *attribute_node_map.get(OBJECT_ATTR).unwrap();
        }

        Ok(res)
    }

    fn add_node(
        &mut self,
        idx: NodeIndex,
        graph: &DiGraphOperators,
        attribute_node_map: &mut HashMap<String, NodeIndex>,
    ) {
        let operator = &graph[idx].operator;
        match operator {
            Operator::ExtendOp { config } => {
                let extend_attribute = config.extend_pairs.keys().next().unwrap();
                attribute_node_map.insert(extend_attribute.to_string(), idx);
                self.node_indices.push(idx);
            }
            _ => self.node_indices.push(idx),
        }
    }
}

pub fn log_delta_trmap_sub_expressions(
    graph: &DiGraphOperators,
    original_trmap_sub_exprs: &[TrMapSubExpression],
    retained_trmap_sub_exprs: &[TrMapSubExpression],
    query_type: &str,
) -> RMLResult<()> {
    let original_log_file = format!("{}_original_trmap.log", query_type);
    log::debug!(
        "Serializing and logging original set of trmap sub expression to: {}",
        original_log_file
    );
    log_trmap_sub_expr_to_file(
        graph,
        original_trmap_sub_exprs,
        original_log_file,
        original_trmap_sub_exprs.len(),
        query_type,
    )?;

    let retained_log_file = format!("{}_retained.log", query_type);
    log::debug!(
        "Serializing and logging retained set of trmap sub expression to: {}",
        retained_log_file
    );
    log_trmap_sub_expr_to_file(
        graph,
        retained_trmap_sub_exprs,
        retained_log_file,
        original_trmap_sub_exprs.len(),
        query_type,
    )?;

    Ok(())
}

fn log_trmap_sub_expr_to_file(
    graph: &DiGraphOperators,
    trmap_sub_exprs: &[TrMapSubExpression],
    log_file: String,
    original_trmap_count: usize,
    query_type: &str,
) -> Result<(), RMLExtractorError> {
    let file = File::create(log_file)?;
    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "{}Start of simple print{}",
        "=".repeat(10),
        "=".repeat(10)
    )?;
    for trmap in trmap_sub_exprs {
        let operators: Vec<_> = trmap
            .node_indices
            .iter()
            .flat_map(|idx| graph.node_weight(*idx))
            .map(|node| match &node.operator {
                Operator::SourceOp { config } => {
                    format!("Source Op with config: {}", config.pretty_string().unwrap())
                }
                Operator::JoinOp { config: _config } => "JoinOp".to_string(),
                Operator::UnionOp => "Union".to_string(),
                Operator::ProjectOp { config: _ } => "Projection".to_string(),
                Operator::ExtendOp { config } => format!("ExtendOperator with config {:?}", config),
                Operator::RenameOp { config: _config } => "RenameOperator".to_string(),
                Operator::SerializerOp { config: _ } => "SerializeOperator".to_string(),
                Operator::TargetOp { config: _ } => "TargetOperator".to_string(),
            })
            .collect();
        writeln!(writer, "[{}]", operators.join(","))?;
    }
    writeln!(writer, "{}", "=".repeat(20))?;

    writeln!(
        writer,
        "Total number of Trmap-sub-expressions for {}: {}",
        query_type,
        trmap_sub_exprs.len()
    )?;
    if trmap_sub_exprs.len() != original_trmap_count {
        writeln!(
            writer,
            "Prune factor for {}: {:.4}",
            query_type,
            original_trmap_count as f32 / trmap_sub_exprs.len() as f32
        )?;
        writeln!(
            writer,
            "Prune percentage factor for {}: {:.4}",
            query_type,
            trmap_sub_exprs.len() as f32 / original_trmap_count as f32
        )?;
    }
    writer.flush()?;
    Ok(())
}
