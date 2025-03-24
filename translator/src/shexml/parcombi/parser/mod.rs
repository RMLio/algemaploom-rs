mod tests;
pub mod r#type;
use std::collections::HashMap;

use chumsky::prelude::*;
use chumsky::Parser;

use self::r#type::*;
use super::lexer::token::*;

macro_rules! shex_just {
    ($exp:expr) => {
        just::<ShExMLToken, _, Simple<ShExMLToken>>($exp)
    };
}

macro_rules! t {
    ($t:ty) => {
        impl Parser<ShExMLToken, $t, Error = Simple<ShExMLToken>> + Clone
    };
}

macro_rules! unfold_token_value {
    ($t:ident) => {
        select! {
         ShExMLToken::$t(string) => string
        }
    };
}

fn token_string<T: AsRef<str> + Clone>(
    tok: ShExMLToken,
    target: T,
) -> t!(String) {
    just(tok).map(move |_| target.as_ref().to_string())
}

fn token_prefix_shex_pns() -> t!(PrefixNameSpace) {
    select! {
        ShExMLToken::BasePrefix => PrefixNameSpace::BasePrefix,
        ShExMLToken::PrefixNS(prefix) => prefix.parse().unwrap(),
    }
    .labelled("parser:token_prefix_shex_pns")
}

pub fn shexml() -> t!(ShExMLDocument) {
    prefixes()
        .then(sources())
        .then(iterators())
        .map(|((prefixes, sources), iters)| (prefixes, sources, iters))
        .then(expressions())
        .then(graph_shapes())
        .map(
            |(
                ((mut prefixes, sources, iterators), expressions),
                graph_shapes,
            )| {
                let mut matchers = Vec::new();
                let mut auto_increments = Vec::new();
                let mut expression_stmts = Vec::new();
                let mut functions = Vec::new();

                for expr in expressions {
                    match expr {
                        ExpressionEnum::ExpressionStmt(stmt) => {
                            expression_stmts.push(stmt)
                        }
                        ExpressionEnum::MatcherExp(matcher) => {
                            matchers.push(matcher)
                        }
                        ExpressionEnum::AutoIncrementExp(auto_increment) => {
                            auto_increments.push(auto_increment)
                        }
                        ExpressionEnum::FunctionExp(function) => {
                            functions.push(function)
                        }
                    }
                }

                // Add a default rdf prefix
                prefixes.push(Prefix {
                    prefix: PrefixNameSpace::NamedPrefix(
                        vocab::rdf::PREFIX.to_string(),
                    ),
                    uri:    vocab::rdf::IRI.to_string(),
                });
                ShExMLDocument {
                    prefixes,
                    sources,
                    iterators,
                    expression_stmts,
                    matchers,
                    auto_increments,
                    functions,
                    graph_shapes,
                }
            },
        )
}

fn shape_ident() -> t!(ShapeIdent) {
    let pn_ln = select! {
        ShExMLToken::ShapeNode{prefix, local} => (prefix, local)
    };

    pn_ln
        .map(|(prefix, local)| {
            let prefix = prefix.parse().unwrap();

            ShapeIdent { prefix, local }
        })
        .labelled("parser:shape_ident")
}

fn graph_shapes() -> t!(Vec<GraphShapes>) {
    let graph_ident = shape_ident();
    let graph = graph_ident
        .then_ignore(
            just(ShExMLToken::SqBrackStart)
                .repeated()
                .at_least(2)
                .at_most(2),
        )
        .then(shapes())
        .then_ignore(
            just(ShExMLToken::SqBrackEnd)
                .repeated()
                .at_least(2)
                .at_most(2),
        )
        .map(|(ident, shapes)| GraphShapes { ident, shapes });

    graph
        .or(shapes().map(|shapes| {
            GraphShapes {
                ident: ShapeIdent::base(),
                shapes,
            }
        }))
        .repeated()
        .at_least(1)
        .labelled("parser:graph_shapes")
}

fn shape_term() -> t!((PrefixNameSpace, String)) {
    select! {
            ShExMLToken::ShapeTerm{prefix, local} =>{
                let p_ns = prefix.parse().unwrap();
                (p_ns, local)
            }

    }
}

fn shapes() -> t!(Vec<Shape>) {
    let shape_expr = just(ShExMLToken::SqBrackStart)
        .ignore_then(shape_expression())
        .then_ignore(just(ShExMLToken::SqBrackEnd));

    let shape_expr_prefix = token_prefix_shex_pns();

    let prefix_shape_pair =
        shape_expr_prefix.then_ignore(just(ShExMLToken::PrefixSep));

    //Subject prefix shape expression parser
    let subj_prefix = prefix_shape_pair.clone().then(shape_expr.clone()).map(
        |(pn, local)| {
            Subject {
                prefix:     pn,
                expression: local,
            }
        },
    );
    let subj_fixed_iri = shape_term().map(|(pn, local)| {
        Subject {
            prefix:     pn,
            expression: ShapeExpression::Static { value: local },
        }
    });

    let subj = subj_prefix.or(subj_fixed_iri);

    // constant iri subject

    let shape_ident = shape_ident();
    let predicate = shape_term()
        .map(|(p_ns, local)| {
            Predicate {
                prefix: p_ns,
                local,
            }
        })
        .or(just(ShExMLToken::Type).map(|_| {
            Predicate {
                prefix: PrefixNameSpace::NamedPrefix("rdf".to_string()),
                local:  "type".to_string(),
            }
        }));

    shape_ident
        .clone()
        .then(subj)
        .then_ignore(just(ShExMLToken::CurlStart))
        .then(
            predicate
                .then(object())
                .then_ignore(just(ShExMLToken::PredicateSplit))
                .repeated()
                .at_least(1),
        )
        .then_ignore(just(ShExMLToken::CurlEnd))
        .map(|((ident, subject), pred_obj_pairs)| {
            let pred_obj_pairs: Vec<(Predicate, Object)> = pred_obj_pairs;

            Shape {
                ident,
                subject,
                pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
            }
        })
        .repeated()
        .at_least(1)
        .labelled("parser:shapes")
}

fn object() -> t!(Object) {
    let prefix =
        token_prefix_shex_pns().then_ignore(just(ShExMLToken::PrefixSep));

    let shape_expr = just(ShExMLToken::SqBrackStart)
        .ignore_then(shape_expression())
        .then_ignore(just(ShExMLToken::SqBrackEnd));

    //Prefixed object iri parsing
    let prefixed_obj_parsed =
        prefix.clone().then(shape_expr.clone()).map(|(pn, expr)| {
            Object {
                prefix:     Some(pn),
                expression: expr,
                language:   None,
                datatype:   None,
            }
        });
    //

    let namespace_ln = select! {
        ShExMLToken::PrefixLN(ln) => ln
    };

    //Fixed IRI object parsing
    let fixed_iri_obj = shape_term().map(|(prefix, ln)| {
        Object {
            prefix:     Some(prefix),
            expression: ShapeExpression::Static { value: ln },
            language:   None,
            datatype:   None,
        }
    });

    // Literal object with shape expression
    let literal_obj_expr = shape_expr.clone().map(|expr| {
        Object {
            prefix:     None,
            expression: expr,
            language:   None,
            datatype:   None,
        }
    });
    //

    let literal_obj_lexed = shape_expr.clone();
    //Datatyped object parsing

    let static_datatype =
        prefix.clone().then(namespace_ln).map(|(prefix, local)| {
            DataType {
                prefix:     Some(prefix),
                local_expr: ShapeExpression::Static { value: local },
            }
        });

    let dynamic_datatype = prefix
        .clone()
        .or_not()
        .then(shape_expr.clone())
        .map(|(prefix, local_expr)| DataType { prefix, local_expr });

    let datatyped = literal_obj_lexed
        .clone()
        .then(dynamic_datatype.or(static_datatype))
        .map(|(expression, datatype)| {
            Object {
                prefix: None,
                expression,
                language: None,
                datatype: Some(datatype),
            }
        });

    //Language tagged object parsing
    let static_language = select! {
        ShExMLToken::LangTag(langtag) => langtag
    }
    .map(|langtag| ShapeExpression::Static { value: langtag });

    let language_tagged = literal_obj_lexed
        .clone()
        .then_ignore(just(ShExMLToken::AtSymb))
        .then(shape_expr.clone().or(static_language))
        .map(|(obj_expr, language_expr)| {
            Object {
                prefix:     None,
                expression: obj_expr,
                language:   Some(language_expr),
                datatype:   None,
            }
        });
    //

    //Linked shape object parsing
    let linked_shapenode = shape_ident().map(|shape_ident| {
        ShapeExpression::Link {
            other_shape_ident: shape_ident,
        }
    });
    let linked_obj = just(ShExMLToken::AtSymb)
        .ignore_then(linked_shapenode)
        .map(|linked_shape| {
            Object {
                prefix:     None,
                expression: linked_shape,
                language:   None,
                datatype:   None,
            }
        });
    //

    choice((datatyped, language_tagged, linked_obj))
        .or(choice((
            prefixed_obj_parsed,
            literal_obj_expr,
            fixed_iri_obj,
        )))
        .labelled("parser:object")
}

fn shape_expression() -> t!(ShapeExpression) {
    // referencing expression
    let reference_expr = unfold_token_value!(Ident)
        .then(
            shex_just!(ShExMLToken::Dot)
                .ignore_then(unfold_token_value!(Ident))
                .repeated()
                .map(|fields| {
                    if fields.is_empty() {
                        None
                    } else {
                        Some(fields.join("."))
                    }
                }),
        )
        .map(|(expr_ident, field)| ShapeReference { expr_ident, field });

    // function application expression
    let func_params = reference_expr
        .clone()
        .separated_by(just(ShExMLToken::Comma));
    let func_expr = reference_expr
        .clone()
        .then(func_params.delimited_by(
            just(ShExMLToken::BrackStart),
            just(ShExMLToken::BrackEnd),
        ))
        .map(|(fun_method_ident, params_idents)| {
            ShapeExpression::Function {
                fun_method_ident,
                params_idents,
            }
        });

    // matching expression
    let matching_expr = reference_expr
        .clone()
        .then_ignore(just(ShExMLToken::Matching))
        .then(unfold_token_value!(Ident))
        .map(|(reference, matcher_ident)| {
            ShapeExpression::Matching {
                reference,
                matcher_ident,
            }
        });

    // conditional expression
    let conditional_expr = reference_expr
        .clone()
        .then_ignore(just(ShExMLToken::If))
        .then(func_expr.clone())
        .map(|(reference, function)| {
            ShapeExpression::Conditional {
                reference,
                conditional_expr: Box::new(function),
            }
        });

    choice((
        conditional_expr,
        func_expr,
        matching_expr,
        reference_expr.map(ShapeExpression::Reference),
    ))
    .labelled("parser:shape_expression")
}

fn expressions() -> t!(Vec<ExpressionEnum>) {
    (expression_stmt()
        .or(matcher())
        .or(function())
        .or(auto_increment()))
    .repeated()
    .at_least(1)
    .labelled("parser:expressions")
}

fn function() -> t!(ExpressionEnum) {
    shex_just!(ShExMLToken::Function)
        .ignore_then(unfold_token_value!(Ident))
        .then(
            unfold_token_value!(FunctionLang)
                .then(unfold_token_value!(URI))
                .delimited_by(
                    just(ShExMLToken::AngleStart),
                    just(ShExMLToken::AngleEnd),
                ),
        )
        .map(|(ident, (lang_type, uri))| {
            let function = Function {
                ident: ident.clone(),
                lang_type,
                uri,
            };
            ExpressionEnum::FunctionExp(function)
        })
        .labelled("parser:function")
}

fn auto_increment() -> t!(ExpressionEnum) {
    let auto_inc_ident_exp = unfold_token_value!(Ident)
        .then_ignore(just(ShExMLToken::AngleStart))
        .then(
            unfold_token_value!(AutoIncPrefix)
                .or_not()
                .then(unfold_token_value!(AutoIncStart))
                .then(unfold_token_value!(AutoIncEnd).or_not())
                .then(unfold_token_value!(AutoIncStep).or_not())
                .then(unfold_token_value!(AutoIncSuffix).or_not()),
        )
        .then_ignore(just(ShExMLToken::AngleEnd))
        .map(|(ident, ((((prefix, start), end), step), suffix))| {
            let auto_increment = AutoIncrement {
                ident: ident.clone(),
                start,
                prefix,
                suffix,
                end,
                step,
            };
            ExpressionEnum::AutoIncrementExp(auto_increment)
        });

    just(ShExMLToken::AutoIncrement)
        .ignore_then(auto_inc_ident_exp)
        .labelled("parser:auto_increment")
}

fn matcher() -> t!(ExpressionEnum) {
    let field_values = unfold_token_value!(Value)
        .chain::<String, _, _>(
            shex_just!(ShExMLToken::Comma)
                .ignore_then(unfold_token_value!(Value))
                .repeated(),
        )
        .then_ignore(just(ShExMLToken::As))
        .then(unfold_token_value!(Ident))
        .map(|(values, key)| (key, values));

    just(ShExMLToken::Matcher)
        .ignore_then(unfold_token_value!(Ident))
        .then::<Vec<(String, Vec<String>)>, _>(
            just(ShExMLToken::AngleStart)
                .ignore_then(field_values.clone())
                .chain(
                    just(ShExMLToken::MatcherSplit)
                        .ignore_then(field_values)
                        .repeated(),
                )
                .then_ignore(just(ShExMLToken::AngleEnd)),
        )
        .map(|(ident, key_values_vec)| {
            let mut rename_map = HashMap::new();

            for (key, values) in key_values_vec {
                rename_map.insert(key, values.into_iter().collect());
            }
            let matcher = Matcher {
                ident: ident.clone(),
                rename_map,
            };

            ExpressionEnum::MatcherExp(matcher)
        })
        .labelled("parser:matcher")
}

fn exp_reference_ident() -> t!(ExpressionReferenceIdent) {
    unfold_token_value!(Ident)
        .then(
            just(ShExMLToken::Dot)
                .ignore_then(unfold_token_value!(Ident))
                .repeated()
                .at_least(1)
                .at_most(2)
                .map(|idents| {
                    let iterator = idents[0].clone();
                    let field = idents.get(1).map(Clone::clone);

                    (iterator, field)
                }),
        )
        .map(|(source_ident, (iterator_ident, field))| {
            ExpressionReferenceIdent {
                source_ident,
                iterator_ident,
                field,
            }
        })
        .labelled("parser:exp_ident")
}

fn expression_stmt() -> t!(ExpressionEnum) {
    just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Expression)
        .ignore_then(unfold_token_value!(Ident))
        .then_ignore(just(ShExMLToken::AngleStart))
        .then(exp_join_union().or(exp_string_op()).or(exp()))
        .then_ignore(just(ShExMLToken::AngleEnd))
        .map(|(ident, expression)| {
            let stmt = ExpressionStmt {
                ident,
                expr_enum: expression,
            };
            ExpressionEnum::ExpressionStmt(stmt)
        })
        .labelled("parser:expression_stmt")
}

fn exp() -> t!(ExpressionStmtEnum) {
    exp_reference_ident()
        .map(|reference| ExpressionStmtEnum::Basic { reference })
}

fn exp_join_union() -> t!(ExpressionStmtEnum) {
    let basic_expression = exp_string_op().or(exp());

    basic_expression
        .clone()
        .then(
            just(ShExMLToken::Union)
                .to(ExpressionStmtEnum::Union as fn(_, _) -> _)
                .or(just(ShExMLToken::Join)
                    .to(ExpressionStmtEnum::Join as fn(_, _) -> _)),
        )
        .repeated()
        .then(basic_expression)
        .foldr(|(lhs, op), rhs| op(Box::new(lhs), Box::new(rhs)))
        .labelled("parser:exp_join_union")
}

fn exp_string_op() -> t!(ExpressionStmtEnum) {
    exp_reference_ident()
        .then(unfold_token_value!(StringSep))
        .then(exp_reference_ident())
        .map(|((left_reference, concate_string), right_reference)| {
            ExpressionStmtEnum::ConcatenateString {
                left_reference,
                concate_string,
                right_reference,
            }
        })
        .labelled("parser:exp_string_op")
}

fn sources() -> t!(Vec<Source>) {
    let protocol = select! {
        ShExMLToken::File => SourceType::File,
        ShExMLToken::HTTP => SourceType::HTTP,
        ShExMLToken::HTTPS => SourceType::HTTPS,
        ShExMLToken::JDBC(jdbc_type) => SourceType::JDBC(jdbc_type),
    };

    let protocol_uri = protocol.then(unfold_token_value!(URI));

    just(ShExMLToken::Source)
        .ignore_then(unfold_token_value!(Ident))
        .then(protocol_uri.delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(ident, (source_type, uri))| {
            Source {
                ident,
                uri,
                source_type,
            }
        })
        .repeated()
        .at_least(1)
        .labelled("parser:sources")
}

fn iterators() -> t!(Vec<Iterator>) {
    let fields = field().repeated().at_least(1);

    recursive(|recurs| {
        just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Iterator)
            .ignore_then(unfold_token_value!(Ident))
            .then(
                unfold_token_value!(IteratorType)
                    .map(|iter| iter.parse::<IteratorType>().unwrap())
                    .or_not()
                    .then(unfold_token_value!(IteratorQuery).or_not())
                    .delimited_by(
                        just(ShExMLToken::AngleStart),
                        just(ShExMLToken::AngleEnd),
                    ),
            )
            .then_ignore(just(ShExMLToken::BrackStart))
            .then(fields)
            .map(|((ident, (iter_type, query)), fields)| {
                // Edge case handling for csvperrow iterator
                if query == Some("csvperrow".to_string()) && iter_type.is_none()
                {
                    (ident, Some(IteratorType::CSVRows), query, fields)
                } else {
                    (ident, iter_type, query, fields)
                }
            })
            .then(recurs.repeated())
            .then_ignore(just(ShExMLToken::BrackEnd).or_not())
            .map(|((ident, iter_type, query, fields), nested_iterator)| {
                Iterator {
                    ident,
                    query,
                    iter_type,
                    fields,
                    nested_iterator,
                }
            })
    })
    .repeated()
    .at_least(1)
    .labelled("parser:iterators")
}

fn field() -> t!(Field) {
    let field_type = select! {
        ShExMLToken::PushField => FieldType::Push,
        ShExMLToken::Field => FieldType::Normal,
        ShExMLToken::PopField => FieldType::Pop,
        _ => FieldType::Normal,
    };

    field_type
        .then(unfold_token_value!(Ident))
        .then(unfold_token_value!(FieldQuery))
        .map(|((field_type, name), query)| {
            Field {
                ident: name,
                query,
                field_type,
            }
        })
        .labelled("parser:field_type")
}

fn prefixes() -> t!(Vec<Prefix>) {
    let prefix_ns = token_prefix_shex_pns();

    just(ShExMLToken::Prefix)
        .ignore_then(prefix_ns)
        .then_ignore(just(ShExMLToken::PrefixSep))
        .then(unfold_token_value!(URI).delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(prefix, uri)| Prefix { prefix, uri })
        .repeated()
        .at_least(1)
        .labelled("parser:prefixes")
}
