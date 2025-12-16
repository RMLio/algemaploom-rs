use std::fmt::Display;

use petgraph::{algo::Cycle, graph::NodeIndex};

pub type RMLResult<T> = Result<T, RMLExtractorError>;

#[derive(Debug)]
pub enum RMLExtractorError {
    CycleError(Cycle<NodeIndex>),
    IoError(std::io::Error),
    NodeNotFound(NodeIndex),
    NeighbourNotFound(NodeIndex),
    SpecialAttributesIncorrectAmount(String),
    IncorrectNeighbourAmountFound {
        start: NodeIndex,
        expected: usize,
        actual: usize,
    },
    CannotCreateRMLSpecificMapping(NodeIndex),
}

impl Display for RMLExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "error occurred while checking for satisfiability\n")?;
        match self {
            RMLExtractorError::CycleError(cycle) => {
                write!(f, "cycle detected in the graph : {:#?}", cycle)
            }
            RMLExtractorError::NodeNotFound(node_index) => {
                write!(f, "node not found in graph: {:#?}", node_index)
            }
            RMLExtractorError::NeighbourNotFound(node_index) => {
                write!(f, "neighbours not found for node: {:#?}", node_index)
            }
            RMLExtractorError::CannotCreateRMLSpecificMapping(node_idx) => {
                write!(
                    f,
                    "cannot create RML specific mappings from the given node: {:#?}",
                    node_idx
                )
            }
            RMLExtractorError::IncorrectNeighbourAmountFound {
                start,
                expected,
                actual,
            } => write!(
                f,
                "not enough neighbours found for node: {:#?}, expected: {}, but found: {}",
                start, expected, actual
            ),
            RMLExtractorError::SpecialAttributesIncorrectAmount(msg) => write!(
                f,
                "incorrect number of special attributes found with msg: {}",
                msg
            ),
            RMLExtractorError::IoError(_error) => write!(f, "io error occurred"),
        }
    }
}

impl From<std::io::Error> for RMLExtractorError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl std::error::Error for RMLExtractorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RMLExtractorError::IoError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<Cycle<NodeIndex>> for RMLExtractorError {
    fn from(value: Cycle<NodeIndex>) -> Self {
        Self::CycleError(value)
    }
}
