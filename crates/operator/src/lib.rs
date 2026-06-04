//! This module defines the operators that can be used in a data transformation pipeline.
//! Each operator is represented as a variant of the `Operator` enum, which contains the
//! configuration for that operator. The operators include Source, Join, Union, Projection, Extend,
//! Rename, Serializer, and Target. Each operator has its own configuration struct that defines
//! the parameters for that operator.
//! The module also includes traits for displaying the operators in JSON and pretty formats.
pub mod display;
pub mod formats;
pub mod join;
pub mod projection;
pub mod rename;
pub mod extend;
pub mod serializer;
pub mod io;
pub mod utils;

use std::rc::Rc;

pub use crate::extend::Extend;
pub use crate::join::Join;
use crate::projection::Projection;
use crate::rename::Rename;
pub use crate::serializer::Serializer;
use anyhow::Result;
use display::{JsonDisplay, PrettyDisplay};
pub use io::source::Source;
pub use io::target::Target;
use serde::{Deserialize, Serialize};

pub type RcOperator = Rc<Operator>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Operator {
    SourceOp {
        config: Source,
    },
    JoinOp {
        config: Join,
    },
    UnionOp,
    ProjectOp {
        config: Projection,
    },
    ExtendOp {
        config: Extend,
    },
    RenameOp {
        config: Rename,
    },
    SerializerOp {
        config: Serializer,
    },
    TargetOp {
        config: Target,
    }
}

impl From<Extend> for Operator {
    fn from(config: Extend) -> Self {
        Self::ExtendOp { config }
    }
}

impl JsonDisplay for Operator {
    fn json_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl PrettyDisplay for Operator {
    fn pretty_string(&self) -> Result<String> {
        let (title_string, content_string) = match self {
            Operator::SourceOp { config } => {
                ("Source Operator".to_string(), config.pretty_string()?)
            }
            Operator::ProjectOp { config } => {
                ("Projection Operator".to_string(), config.pretty_string()?)
            }
            Operator::ExtendOp { config } => {
                ("Extension Operator".to_string(), config.pretty_string()?)
            }
            Operator::RenameOp { config } => {
                ("Rename Operator".to_string(), config.pretty_string()?)
            }
            Operator::SerializerOp { config } => {
                ("Serializer Operator".to_string(), config.pretty_string()?)
            }
            Operator::TargetOp { config } => {
                ("Target Operator".to_string(), config.pretty_string()?)
            }
            Operator::JoinOp { config } => {
                ("Join Operator".to_string(), config.pretty_string()?)
            }
            Operator::UnionOp => ("Union Operator".to_string(), "".to_string()),
        };

        Ok(format!("{}\n{}", title_string, content_string))
    }
}