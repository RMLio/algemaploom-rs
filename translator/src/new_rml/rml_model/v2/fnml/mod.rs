use sophia_term::RcTerm;

use super::core::expression_map::term_map::TermMap;

#[derive(Debug, Clone)]
pub struct FunctionExecution {
    pub function: RcTerm,
    pub input:    Vec<InputMap>,
}

#[derive(Debug, Clone)]
pub struct InputMap {
    pub parameter: RcTerm,
    pub value_map: TermMap,
}
