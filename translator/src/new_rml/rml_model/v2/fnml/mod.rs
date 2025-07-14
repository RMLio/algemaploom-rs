use sophia_term::RcTerm;

use super::{core::expression_map::term_map::CommonTermMapInfo, AttributeAliaser};

#[derive(Debug, Clone, Hash)]
pub struct FunctionExecution {
    pub function: RcTerm,
    pub input:    Vec<InputMap>,
}

impl AttributeAliaser for FunctionExecution{
    fn alias_attribute(&self, alias: &str) -> Self {
        Self{
            function: self.function.clone(), 
            input: self.input.iter().map(|val| val.alias_attribute(alias)).collect()
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct InputMap {
    pub parameter: RcTerm,
    pub value_map: CommonTermMapInfo,
}

impl AttributeAliaser for InputMap{
    fn alias_attribute(&self, alias: &str) -> Self {
        Self{
            parameter: self.parameter.clone(),
            value_map: self.value_map.alias_attribute(alias),
        }
    }
}
