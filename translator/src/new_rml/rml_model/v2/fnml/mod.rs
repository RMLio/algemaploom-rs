use sophia_term::RcTerm;

use super::core::expression_map::term_map::CommonTermMapInfo;
use super::{AttributeAliaser, RefAttributeGetter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionExpressionMap {
    pub return_map:     Option<Box<CommonTermMapInfo>>,
    pub func_execution: FunctionExecution,
}

impl AttributeAliaser for FunctionExpressionMap {
    fn alias_attribute(&self, alias: &str) -> Self {
        todo!()
    }
}
impl RefAttributeGetter for FunctionExpressionMap {
    fn get_ref_attributes(&self) -> std::collections::HashSet<String> {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionMap {
    pub term_map_info: CommonTermMapInfo,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FunctionExecution {
    pub function_map: Box<FunctionMap>,
    pub input:        Vec<InputMap>,
}

impl AttributeAliaser for FunctionExecution {
    fn alias_attribute(&self, alias: &str) -> Self {
        Self {
            function_map: self.function_map.clone(),
            input:        self
                .input
                .iter()
                .map(|val| val.alias_attribute(alias))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InputMap {
    pub parameter_map:   CommonTermMapInfo,
    pub input_value_map: CommonTermMapInfo,
}

impl AttributeAliaser for InputMap {
    fn alias_attribute(&self, alias: &str) -> Self {
        Self {
            parameter_map:   self.parameter_map.clone(),
            input_value_map: self.input_value_map.alias_attribute(alias),
        }
    }
}
