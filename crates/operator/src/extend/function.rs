use crate::extend::term_type::TermType;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub type RcExtendFunction = Rc<Function>;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Function {
    Nop,
    SimpleConcat {
        inner_function: Option<RcExtendFunction>,
    },
    Concatenate {
        left_value:  RcExtendFunction,
        separator:   String,
        right_value: RcExtendFunction,
    },
    Reference {
        value: String,
    },
    TypedConstant {
        value:     String,
        term_type: TermType,
    },

    Constant {
        value: String,
    },
    TemplateString {
        value: String,
    },

    Replace {
        replace_map:    HashMap<String, HashSet<String>>,
        inner_function: RcExtendFunction,
    },

    TemplateFunctionValue {
        template:                String,
        variable_function_pairs: Vec<(String, RcExtendFunction)>,
    },
    IriEncode {
        inner_function: RcExtendFunction,
    },
    UriEncode {
        inner_function: RcExtendFunction,
    },
    Iri {
        base_iri:       Option<String>,
        inner_function: RcExtendFunction,
    },
    Literal {
        inner_function:    RcExtendFunction,
        dtype_function:    Option<RcExtendFunction>,
        langtype_function: Option<RcExtendFunction>,
    },
    BlankNode {
        inner_function: RcExtendFunction,
    },
    Upper {
        inner_function: RcExtendFunction,
    },
    Lower {
        inner_function: RcExtendFunction,
    },
    FnO {
        fno_identifier: String,
        parameters:     HashMap<String, RcExtendFunction>,
        return_type:    Option<String>,
    },
    Star {
        // TODO: Implement star function
    },
}
