use std::rc::Rc;

use log::warn;
use operator::Function;
use shexml_interpreter::{
    IndexedShExMLDocument, Object, PrefixNameSpace, ShapeExpression,
};

pub fn obj_lang_datatype_function(
    doc: &IndexedShExMLDocument,
    obj: &Object,
) -> Option<Function> {
    let obj_function_opt =
        rdf_term_function(doc, obj.prefix.as_ref(), &obj.expression);

    let obj_inner_function = obj_function_opt?;
    if let Some(obj_prefix) = &obj.prefix {
        if obj_prefix == &PrefixNameSpace::BNodePrefix {
            Some(Function::BlankNode {
                inner_function: obj_inner_function.into(),
            })
        } else {
            Some(Function::Iri {
                base_iri:None,
                inner_function: obj_inner_function.into(),
            })
        }
    } else {
        let dtype_function = obj.datatype.as_ref().and_then(|dtype| {
            rdf_term_function(doc, dtype.prefix.as_ref(), &dtype.local_expr)
                .map(|fun| fun.into())
        });

        let langtype_function = obj.language.as_ref().and_then(|lang_expr| {
            rdf_term_function(doc, None, lang_expr).map(|fun| fun.into())
        });

        Some(Function::Literal {
            inner_function: obj_inner_function.into(),
            dtype_function,
            langtype_function,
        })
    }
}

pub fn rdf_term_function(
    doc: &IndexedShExMLDocument,
    prefix_ns_opt: Option<&PrefixNameSpace>,
    shape_expression: &ShapeExpression,
) -> Option<Function> {
    let function_value_opt = match shape_expression {
        ShapeExpression::Reference(reference) => {
            Some(Function::Reference {
                value: reference.to_string(),
            })
        }
        ShapeExpression::Matching {
            reference,
            matcher_ident,
        } => {
            let ref_func = Function::Reference {
                value: reference.to_string(),
            };

            doc.matchers.get(matcher_ident).map(|matcher| {
                Function::Replace {
                    replace_map:    matcher.rename_map.clone(),
                    inner_function: ref_func.into(),
                }
            })
        }

        ShapeExpression::Link { other_shape_ident } => {
            Some(Function::Reference {
                value: other_shape_ident.to_string(),
            })
        }

        ShapeExpression::Static { value } => {
            Some(Function::Constant {
                value: value.to_string(),
            })
        }

        shape_expression => {
            warn!(
                "Extracting external functions from shape expression: {:#?} is not supported",
                shape_expression
            );

            None
        }
    };
    //Still need to handle templating if there is a prefix
    if let Some(new_func) = function_value_opt {
        if let Some(prefix_ns) = prefix_ns_opt {
            let prefix_opt = doc.prefixes.get(&prefix_ns.to_string());
            if let Some(prefix) = prefix_opt {
                let template = format!("{}{{func_value}}", prefix.uri,);

                return Some(Function::TemplateFunctionValue {
                    template,
                    variable_function_pairs: vec![(
                        "func_value".to_string(),
                        Rc::new(new_func),
                    )],
                });
            } else if prefix_ns == &PrefixNameSpace::BNodePrefix {
                return Some(new_func);
            }
        } else {
            return Some(new_func);
        }
    }

    None
}
