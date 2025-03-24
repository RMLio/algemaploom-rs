pub const PREFIX: &str = "rml";

pub const IRI: &str = "http://w3id.org/rml/";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const RETURN_MAP: PAIR = (IRI, "returnMap");
    pub const RETURN: PAIR = (IRI, "return");
    pub const FUNCTION_EXECUTION: PAIR = (IRI, "functionExecution");
    pub const INPUT: PAIR = (IRI, "input");
    pub const FUNCTION_MAP: PAIR = (IRI, "functionMap");
    pub const FUNCTION: PAIR = (IRI, "function");

    pub const PARAMETER_MAP: PAIR = (IRI, "parameterMap");
    pub const PARAMETER: PAIR = (IRI, "parameter"); 
    pub const INPUT_VALUE_MAP: PAIR = (IRI, "inputValueMap");
    pub const INPUT_VALUE: PAIR = (IRI, "inputValue");
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const PARAMETER_MAP: PAIR = (IRI, "ParameterMap");
    pub const FUNCTION_MAP: PAIR = (IRI, "FunctionMap");
    pub const INPUT: PAIR = (IRI, "Input");
    pub const RETURN_MAP: PAIR = (IRI, "ReturnMap");
    pub const FUNCTION_EXECUTION: PAIR = (IRI, "FunctionExecution");
}
