#[cfg(test)]
use std::collections::HashSet;
use crate::shexml::parcombi::parser;
use crate::shexml::parcombi::lexer;
use crate::shexml::parcombi::*; 
use chumsky::prelude::*; 

 

fn assert_parse_expected<T: std::fmt::Debug + PartialEq + Eq>(
    parsed_items: Option<T>,
    expected_items: Option<T>,
) {
    assert!(
        parsed_items == expected_items,
        "{:#?} is the parsed items
            {:#?} is the expected items
            ",
        parsed_items,
        expected_items
    );
}

#[test]
fn shexml_simple_test() {
    let document_str = "
PREFIX : <http://example.com/>
SOURCE films_xml_file <https://rawgit.com/herminiogg/ShExML/master/src/test/resources/films.xml>
SOURCE films_json_file <https://rawgit.com/herminiogg/ShExML/master/src/test/resources/films.json>
ITERATOR film_xml <xpath: //film> {
    FIELD id <@id>
    FIELD name <name>
    FIELD year <year>
    FIELD country <country>
    FIELD directors <directors/director>
}
ITERATOR film_json <jsonpath: $.films[*]> {
    FIELD id <id>
    FIELD name <name>
    FIELD year <year>
    FIELD country <country>
    FIELD directors <director>
}
EXPRESSION films <films_xml_file.film_xml UNION films_json_file.film_json>

:Films :[films.id] {
    :name [films.name] ;
    :year [films.year] ;
    :country [films.country] ;
    :director [films.directors] ;
}
        ";

    let (tokens_opt, errors) = lexer::shexml().parse_recovery(document_str);

    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shexml().parse_recovery_verbose(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(parsed_items.is_some(), "{:?}", parsed_items);
}

#[test]
fn multiple_graph_test() {
    let mutli_graph_str = " 
:BaseGraph [[ 
    :Films :[films.id IF helper.isBefore2010(films.year)] {
        :name [films.name] ;
        :countryOfOrigin [films.country IF helper.outsideUSA(films.country)] ;
    }

]]

:AnotherGraph [[ 
    :Films2 :[films.id IF helper.isBefore2010(films.year)] {
        :year :[films.year] ;
    }
]]
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(mutli_graph_str);

    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::graph_shapes().parse_recovery(tokens_opt.unwrap());

    let subject = Subject {
        prefix:     PrefixNameSpace::BasePrefix,
        expression: ShapeExpression::Conditional {
            reference:        ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            },
            conditional_expr: Box::new(ShapeExpression::Function {
                fun_method_ident: ShapeReference {
                    expr_ident: "helper".to_string(),
                    field:      Some("isBefore2010".to_string()),
                },
                params_idents:    vec![ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }],
            }),
        },
    };

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "countryOfOrigin".to_string(),
            },
            Object {
                prefix:     None,
                language:   None,
                datatype:   None,
                expression: ShapeExpression::Conditional {
                    reference:        ShapeReference {
                        expr_ident: "films".to_string(),
                        field:      Some("country".to_string()),
                    },
                    conditional_expr: Box::new(ShapeExpression::Function {
                        fun_method_ident: ShapeReference {
                            expr_ident: "helper".to_string(),
                            field:      Some("outsideUSA".to_string()),
                        },
                        params_idents:    vec![ShapeReference {
                            expr_ident: "films".to_string(),
                            field:      Some("country".to_string()),
                        }],
                    }),
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
    ];

    let subject_2 = subject.clone();
    let pred_obj_pairs_2 = vec![(
        Predicate {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "year".to_string(),
        },
        Object {
            language:   None,
            datatype:   None,
            prefix:     Some(PrefixNameSpace::BasePrefix),
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("year".to_string()),
            }),
        },
    )];

    let shape = Shape {
        ident: ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject,
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    };

    let shape_2 = Shape {
        ident: ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films2".to_string(),
        },

        subject:        subject_2,
        pred_obj_pairs: pred_obj_pairs_2.into_iter().collect(),
    };

    let expected_items = Some(vec![
        GraphShapes {
            ident:  ShapeIdent {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "BaseGraph".to_string(),
            },
            shapes: vec![shape],
        },
        GraphShapes {
            ident:  ShapeIdent {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "AnotherGraph".to_string(),
            },
            shapes: vec![shape_2],
        },
    ]);

    assert!(errors.len() == 0, "{:?}", errors);
    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn graph_multiple_shapes_test() {
    let graph_shape_str = "  
:BaseGraph [[ 
    :Films :[films.id IF helper.isBefore2010(films.year)] {
        :name [films.name] ;
        :countryOfOrigin [films.country IF helper.outsideUSA(films.country)] ;
    }

    :Films2 :[films.id IF helper.isBefore2010(films.year)] {
        :year :[films.year] ;
    }
]]
        ";
    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(graph_shape_str);

    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::graph_shapes().parse_recovery(tokens_opt.unwrap());

    let subject = Subject {
        prefix:     PrefixNameSpace::BasePrefix,
        expression: ShapeExpression::Conditional {
            reference:        ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            },
            conditional_expr: Box::new(ShapeExpression::Function {
                fun_method_ident: ShapeReference {
                    expr_ident: "helper".to_string(),
                    field:      Some("isBefore2010".to_string()),
                },
                params_idents:    vec![ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }],
            }),
        },
    };

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "countryOfOrigin".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Conditional {
                    reference:        ShapeReference {
                        expr_ident: "films".to_string(),
                        field:      Some("country".to_string()),
                    },
                    conditional_expr: Box::new(ShapeExpression::Function {
                        fun_method_ident: ShapeReference {
                            expr_ident: "helper".to_string(),
                            field:      Some("outsideUSA".to_string()),
                        },
                        params_idents:    vec![ShapeReference {
                            expr_ident: "films".to_string(),
                            field:      Some("country".to_string()),
                        }],
                    }),
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
    ];

    let subject_2 = subject.clone();
    let pred_obj_pairs_2 = vec![(
        Predicate {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "year".to_string(),
        },
        Object {
            language:   None,
            datatype:   None,
            prefix:     Some(PrefixNameSpace::BasePrefix),
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("year".to_string()),
            }),
        },
    )];

    let shape = Shape {
        ident: ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject,
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    };

    let shape_2 = Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films2".to_string(),
        },
        subject:        subject_2,
        pred_obj_pairs: pred_obj_pairs_2.into_iter().collect(),
    };

    let expected_items = Some(vec![GraphShapes {
        ident:  ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "BaseGraph".to_string(),
        },
        shapes: vec![shape, shape_2],
    }]);

    assert!(errors.len() == 0, "{:?}", errors);
    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn graph_shape_test() {
    let graph_shape_str = "  
:BaseGraph [[ 
    :Films :[films.id IF helper.isBefore2010(films.year)] {
        :name [films.name] ;
        :year :[films.year] ;
        :countryOfOrigin [films.country IF helper.outsideUSA(films.country)] ;
    }
]]
        ";
    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(graph_shape_str);

    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::graph_shapes().parse_recovery(tokens_opt.unwrap());

    let subject = Subject {
        prefix:     PrefixNameSpace::BasePrefix,
        expression: ShapeExpression::Conditional {
            reference:        ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            },
            conditional_expr: Box::new(ShapeExpression::Function {
                fun_method_ident: ShapeReference {
                    expr_ident: "helper".to_string(),
                    field:      Some("isBefore2010".to_string()),
                },
                params_idents:    vec![ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }],
            }),
        },
    };

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "countryOfOrigin".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Conditional {
                    reference:        ShapeReference {
                        expr_ident: "films".to_string(),
                        field:      Some("country".to_string()),
                    },
                    conditional_expr: Box::new(ShapeExpression::Function {
                        fun_method_ident: ShapeReference {
                            expr_ident: "helper".to_string(),
                            field:      Some("outsideUSA".to_string()),
                        },
                        params_idents:    vec![ShapeReference {
                            expr_ident: "films".to_string(),
                            field:      Some("country".to_string()),
                        }],
                    }),
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::BasePrefix),
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let shape = Shape {
        ident: ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject,
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    };

    let expected_items = Some(vec![GraphShapes {
        ident:  ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "BaseGraph".to_string(),
        },
        shapes: vec![shape],
    }]);

    assert!(errors.len() == 0, "{:?}", errors);
    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn shape_condition_if_test() {
    let shape_str = "

:Films :[films.id IF helper.isBefore2010(films.year)] {
    :name [films.name] ;
    :year :[films.year] ;
    :countryOfOrigin [films.country IF helper.outsideUSA(films.country)] ;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let subject = Subject {
        prefix:     PrefixNameSpace::BasePrefix,
        expression: ShapeExpression::Conditional {
            reference:        ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            },
            conditional_expr: Box::new(ShapeExpression::Function {
                fun_method_ident: ShapeReference {
                    expr_ident: "helper".to_string(),
                    field:      Some("isBefore2010".to_string()),
                },
                params_idents:    vec![ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }],
            }),
        },
    };

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "countryOfOrigin".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Conditional {
                    reference:        ShapeReference {
                        expr_ident: "films".to_string(),
                        field:      Some("country".to_string()),
                    },
                    conditional_expr: Box::new(ShapeExpression::Function {
                        fun_method_ident: ShapeReference {
                            expr_ident: "helper".to_string(),
                            field:      Some("outsideUSA".to_string()),
                        },
                        params_idents:    vec![ShapeReference {
                            expr_ident: "films".to_string(),
                            field:      Some("country".to_string()),
                        }],
                    }),
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::BasePrefix),
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident: ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject,
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_function_test() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] ;
    :year :[films.year] ;
    :bigName dbr:[helper.allCapitals(films.name)] ;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "bigName".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::NamedPrefix(
                    "dbr".to_string(),
                )),
                expression: ShapeExpression::Function {
                    fun_method_ident: ShapeReference {
                        expr_ident: "helper".to_string(),
                        field:      Some("allCapitals".to_string()),
                    },
                    params_idents:    vec![ShapeReference {
                        expr_ident: "films".to_string(),
                        field:      Some("name".to_string()),
                    }],
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::BasePrefix),
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_matching_test() {
    let shape_str = "
:Films :[films.id] {
    :name dbr:[films.name MATCHING trump];
    :year [films.year] xsd:datetime ;
    :actors @:Actors; 
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "actors".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Link {
                    other_shape_ident: ShapeIdent {
                        prefix: PrefixNameSpace::BasePrefix,
                        local:  "Actors".to_string(),
                    },
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::NamedPrefix(
                    "dbr".to_string(),
                )),
                expression: ShapeExpression::Matching {
                    reference:     ShapeReference {
                        expr_ident: "films".to_string(),
                        field:      Some("name".to_string()),
                    },
                    matcher_ident: "trump".to_string(),
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   Some(DataType {
                    prefix:     Some(PrefixNameSpace::NamedPrefix(
                        "xsd".to_string(),
                    )),
                    local_expr: ShapeExpression::Static {
                        value: "datetime".to_string(),
                    },
                }),
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_multiple_link_test() {
    let shape_str = "
:Films :[films.id] {
    :name [films.name] @en ;
    :year [films.year] xsd:datetime ;
    :actors @:Actors; 
}

:Actors :[films.actor.id]{
    :name [films.actor.name] @en ;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "actors".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Link {
                    other_shape_ident: ShapeIdent {
                        prefix: PrefixNameSpace::BasePrefix,
                        local:  "Actors".to_string(),
                    },
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   Some(ShapeExpression::Static {
                    value: "en".to_string(),
                }),
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   Some(DataType {
                    prefix:     Some(PrefixNameSpace::NamedPrefix(
                        "xsd".to_string(),
                    )),
                    local_expr: ShapeExpression::Static {
                        value: "datetime".to_string(),
                    },
                }),
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];
    let film_shape = Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    };

    let pred_obj_pairs = vec![(
        Predicate {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "name".to_string(),
        },
        Object {
            prefix:     None,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("actor.name".to_string()),
            }),
            language:   Some(ShapeExpression::Static {
                value: "en".to_string(),
            }),
            datatype:   None,
        },
    )];

    let actor_shape = Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Actors".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("actor.id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    };

    let expected_items = Some(vec![film_shape, actor_shape]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_link_test() {
    let shape_str = "
:Films :[films.id] {
    :name [films.name] @en ;
    :year [films.year] xsd:datetime ;
    :actors @:Actors; 
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "actors".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Link {
                    other_shape_ident: ShapeIdent {
                        prefix: PrefixNameSpace::BasePrefix,
                        local:  "Actors".to_string(),
                    },
                },
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   Some(ShapeExpression::Static {
                    value: "en".to_string(),
                }),
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   Some(DataType {
                    prefix:     Some(PrefixNameSpace::NamedPrefix(
                        "xsd".to_string(),
                    )),
                    local_expr: ShapeExpression::Static {
                        value: "datetime".to_string(),
                    },
                }),
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_object_literal_test() {
    let shape_str = "
:Films :[films.id] {
    :name :Jackie; 
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![(
        Predicate {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "name".to_string(),
        },
        Object {
            language:   None,
            datatype:   None,
            prefix:     Some(PrefixNameSpace::BasePrefix),
            expression: ShapeExpression::Static {
                value: "Jackie".to_string(),
            },
        },
    )];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_static_datatype_languagetag_test() {
    let shape_str = "
:Films :[films.id] {
    :name [films.name] @en ;
    :year [films.year] xsd:datetime ;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   Some(ShapeExpression::Static {
                    value: "en".to_string(),
                }),
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   Some(DataType {
                    prefix:     Some(PrefixNameSpace::NamedPrefix(
                        "xsd".to_string(),
                    )),
                    local_expr: ShapeExpression::Static {
                        value: "datetime".to_string(),
                    },
                }),
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_subject_fixed_test() {
    let shape_str = "
:Films :films1 {
    a :Film;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![(
        Predicate {
            prefix: PrefixNameSpace::NamedPrefix("rdf".to_string()),
            local:  "type".to_string(),
        },
        Object {
            language:   None,
            datatype:   None,
            prefix:     Some(PrefixNameSpace::BasePrefix),
            expression: ShapeExpression::Static {
                value: "Film".to_string(),
            },
        },
    )];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Static {
                value: "films1".to_string(),
            },
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_class_type_test() {
    let shape_str = "
:Films :[films.id] {
    a :Film;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![(
        Predicate {
            prefix: PrefixNameSpace::NamedPrefix("rdf".to_string()),
            local:  "type".to_string(),
        },
        Object {
            language:   None,
            datatype:   None,
            prefix:     Some(PrefixNameSpace::BasePrefix),
            expression: ShapeExpression::Static {
                value: "Film".to_string(),
            },
        },
    )];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_bnode_test() {
    let shape_str = "

:Films _:[films.id] {
    :name [films.name] ;
    :year _:[films.year] ;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::BNodePrefix),
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BNodePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn shape_simple_test() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] ;
    :year :[films.year] ;
}
        ";

    let (tokens_opt, errors) = lexer::shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (parsed_items, errors) =
        parser::shapes().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let pred_obj_pairs = vec![
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "name".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     None,
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("name".to_string()),
                }),
            },
        ),
        (
            Predicate {
                prefix: PrefixNameSpace::BasePrefix,
                local:  "year".to_string(),
            },
            Object {
                language:   None,
                datatype:   None,
                prefix:     Some(PrefixNameSpace::BasePrefix),
                expression: ShapeExpression::Reference(ShapeReference {
                    expr_ident: "films".to_string(),
                    field:      Some("year".to_string()),
                }),
            },
        ),
    ];

    let expected_items = Some(vec![Shape {
        ident:          ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "Films".to_string(),
        },
        subject:        Subject {
            prefix:     PrefixNameSpace::BasePrefix,
            expression: ShapeExpression::Reference(ShapeReference {
                expr_ident: "films".to_string(),
                field:      Some("id".to_string()),
            }),
        },
        pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn expressions_test() {
    let expressions_str = " 
        MATCHER ast <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias>

        AUTOINCREMENT myId <2>   

        FUNCTIONS helper <scala: https://raw.githubusercontent.com/herminiogg/ShExML/enhancement-%23121/src/test/resources/functions.scala>

        EXPRESSION exp <file.it1.name UNION file.it2.name JOIN file.it3.name>
        ";

    let expressions_lexer = lexer::expressions();

    let (tokens_opt, errors) = expressions_lexer
        .then_ignore(end())
        .parse_recovery(expressions_str);
    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    let (expressions, errors) =
        parser::expressions().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let source_ident = "file".to_string();
    let field_ident = Some("name".to_string());

    let union_exp = Box::new(ExpressionStmtEnum::Join(
        Box::new(ExpressionStmtEnum::Basic {
            reference: ExpressionReferenceIdent {
                source_ident:   source_ident.clone(),
                iterator_ident: "it2".to_string(),
                field:          field_ident.clone(),
            },
        }),
        Box::new(ExpressionStmtEnum::Basic {
            reference: ExpressionReferenceIdent {
                source_ident:   source_ident.clone(),
                iterator_ident: "it3".to_string(),
                field:          field_ident.clone(),
            },
        }),
    ));
    let expr_enum = ExpressionStmtEnum::Union(
        Box::new(ExpressionStmtEnum::Basic {
            reference: ExpressionReferenceIdent {
                source_ident:   source_ident.clone(),
                iterator_ident: "it1".to_string(),
                field:          field_ident.clone(),
            },
        }),
        union_exp,
    );

    let expected_stmt = ExpressionStmt {
        ident: "exp".to_string(),
        expr_enum,
    };

    let values_set = HashSet::from_iter(vec![
        "Principality of Asturias".to_string(),
        "Principado de Asturias".to_string(),
        "Principáu d'Asturies".to_string(),
        "Asturies".to_string(),
    ]);

    let expected_matcher = Matcher {
        ident:      "ast".to_string(),
        rename_map: vec![("Asturias".to_string(), values_set)]
            .into_iter()
            .collect(),
    };

    let expected_function = Function {
        ident:     "helper".to_string(),
        lang_type: "scala:".to_string(),
        uri:       "https://raw.githubusercontent.com/herminiogg/ShExML/enhancement-%23121/src/test/resources/functions.scala".to_string(),
    };

    let expected_autoinc = AutoIncrement {
        ident:  "myId".to_string(),
        start:  2,
        prefix: None,
        suffix: None,
        end:    None,
        step:   None,
    };

    for exp in expressions.unwrap() {
        match exp {
            ExpressionEnum::ExpressionStmt(stmt) => {
                assert!(stmt == expected_stmt, "{:?}", stmt)
            }
            ExpressionEnum::MatcherExp(matcher) => {
                assert!(matcher == expected_matcher, "{:?}", matcher)
            }
            ExpressionEnum::AutoIncrementExp(autoinc) => {
                assert!(autoinc == expected_autoinc, "{:?}", autoinc)
            }
            ExpressionEnum::FunctionExp(function) => {
                assert!(function == expected_function, "{:?}", function)
            }
        }
    }
}

#[test]
fn function_test() {
    let function_str = "
        FUNCTIONS helper <scala: https://raw.githubusercontent.com/herminiogg/ShExML/enhancement-%23121/src/test/resources/functions.scala>
        ";
    let (tokens_opt, errors) = lexer::function()
        .padded()
        .then_ignore(end())
        .parse_recovery(function_str);

    assert!(errors.is_empty(), "{:?}", errors);
    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::function().parse_recovery(tokens_opt.unwrap());

    assert!(errors.is_empty(), "{:?}", errors);

    let expected_items = Some(

                              ExpressionEnum::FunctionExp(
                              Function {
        ident:     "helper".to_string(),
        lang_type: "scala:".to_string(),
        uri:       "https://raw.githubusercontent.com/herminiogg/ShExML/enhancement-%23121/src/test/resources/functions.scala".to_string(),
    }));

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn auto_inc_only_start_test() {
    let match_str = "
     AUTOINCREMENT myId <2>   
     ";

    let (tokens_opt, errors) = lexer::autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    assert!(errors.is_empty(), "{:?}", errors);

    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::auto_increment().parse_recovery(tokens_opt.unwrap());

    assert!(errors.is_empty(), "{:?}", errors);

    let expected_items =
        Some(ExpressionEnum::AutoIncrementExp(AutoIncrement {
            ident:  "myId".to_string(),
            start:  2,
            prefix: None,
            suffix: None,
            end:    None,
            step:   None,
        }));

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn auto_inc_start_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 >   
     ";

    let (tokens_opt, errors) = lexer::autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);
    assert!(errors.is_empty(), "{:?}", errors);

    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::auto_increment().parse_recovery(tokens_opt.unwrap());

    assert!(errors.is_empty(), "{:?}", errors);

    let expected_items =
        Some(ExpressionEnum::AutoIncrementExp(AutoIncrement {
            ident:  "myId".to_string(),
            start:  0,
            prefix: Some("my".to_string()),
            suffix: None,
            end:    None,
            step:   None,
        }));

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn auto_inc_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 to 10 by 2 + \"Id\">   
     ";

    let (tokens_opt, errors) = lexer::autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    assert!(errors.is_empty(), "{:?}", errors);
    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::auto_increment().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let expected_items =
        Some(ExpressionEnum::AutoIncrementExp(AutoIncrement {
            ident:  "myId".to_string(),
            start:  0,
            prefix: Some("my".to_string()),
            suffix: Some("Id".to_string()),
            end:    Some(10),
            step:   Some(2),
        }));

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn matcher_multiple_test() {
    let match_str = "
        MATCHER regions <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias &
                Spain, España, Espagne AS Spain>
        ";

    let (tokens_opt, errors) = lexer::matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let (parsed_items, errors) =
        parser::matcher().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let asturias_set: HashSet<_> = HashSet::from_iter(vec![
        "Principality of Asturias".to_string(),
        "Principado de Asturias".to_string(),
        "Principáu d'Asturies".to_string(),
        "Asturies".to_string(),
    ]);

    let spain_set: HashSet<_> = HashSet::from_iter(vec![
        "Spain".to_string(),
        "Espagne".to_string(),
        "España".to_string(),
    ]);

    let expected_items = Some(ExpressionEnum::MatcherExp(Matcher {
        ident:      "regions".to_string(),
        rename_map: vec![
            ("Asturias".to_string(), asturias_set),
            ("Spain".to_string(), spain_set),
        ]
        .into_iter()
        .collect(),
    }));

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn matcher_single_test() {
    let match_str = "
        MATCHER ast <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias>
        ";

    let (tokens_opt, errors) = lexer::matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    assert!(errors.len() == 0, "{:?}", errors);
    let (parsed_items, errors) =
        parser::matcher().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let values_set = HashSet::from_iter(vec![
        "Principality of Asturias".to_string(),
        "Principado de Asturias".to_string(),
        "Principáu d'Asturies".to_string(),
        "Asturies".to_string(),
    ]);

    let expected_items = Some(ExpressionEnum::MatcherExp(Matcher {
        ident:      "ast".to_string(),
        rename_map: vec![("Asturias".to_string(), values_set)]
            .into_iter()
            .collect(),
    }));

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn expression_join_union_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.name UNION file.it2.name JOIN file.it3.name>
        ";

    let (tokens_opt, errors) = lexer::expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::expression_stmt().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let source_ident = "file".to_string();
    let field_ident = Some("name".to_string());

    let union_exp = Box::new(ExpressionStmtEnum::Join(
        Box::new(ExpressionStmtEnum::Basic {
            reference: ExpressionReferenceIdent {
                source_ident:   source_ident.clone(),
                iterator_ident: "it2".to_string(),
                field:          field_ident.clone(),
            },
        }),
        Box::new(ExpressionStmtEnum::Basic {
            reference: ExpressionReferenceIdent {
                source_ident:   source_ident.clone(),
                iterator_ident: "it3".to_string(),
                field:          field_ident.clone(),
            },
        }),
    ));
    let expr_enum = ExpressionStmtEnum::Union(
        Box::new(ExpressionStmtEnum::Basic {
            reference: ExpressionReferenceIdent {
                source_ident:   source_ident.clone(),
                iterator_ident: "it1".to_string(),
                field:          field_ident.clone(),
            },
        }),
        union_exp,
    );

    let expected_items = Some(ExpressionEnum::ExpressionStmt(ExpressionStmt {
        ident: "exp".to_string(),
        expr_enum,
    }));

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn expression_simple_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.name>
        ";

    let (tokens_opt, errors) = lexer::expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);
    assert!(errors.len() == 0, "{:?}", errors);

    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::expression_stmt().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let source_ident = "file".to_string();
    let field_ident = Some("name".to_string());
    let expr_enum = ExpressionStmtEnum::Basic {
        reference: ExpressionReferenceIdent {
            source_ident,
            iterator_ident: "it1".to_string(),
            field: field_ident,
        },
    };

    let expected_items = Some(ExpressionEnum::ExpressionStmt(ExpressionStmt {
        ident: "exp".to_string(),
        expr_enum,
    }));

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn expression_string_op_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id + \"-seper-\" +  file.it2.name>
        ";

    let (tokens_opt, errors) = lexer::expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::expression_stmt().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let source_ident = "file".to_string();

    let expr_enum = ExpressionStmtEnum::ConcatenateString {
        left_reference:  ExpressionReferenceIdent {
            source_ident:   source_ident.clone(),
            iterator_ident: "it1".to_string(),
            field:          Some("id".to_string()),
        },
        concate_string:  "-seper-".to_string(),
        right_reference: ExpressionReferenceIdent {
            source_ident:   source_ident.clone(),
            iterator_ident: "it2".to_string(),
            field:          Some("name".to_string()),
        },
    };
    let stmt = ExpressionStmt {
        ident: "exp".to_string(),
        expr_enum,
    };

    let expected_items = Some(ExpressionEnum::ExpressionStmt(stmt));

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn iterator_nested_test() {
    let iter_str = "
    ITERATOR example <jsonpath: $> {
    PUSHED_FIELD field1 <id>
    ITERATOR nestedIterator <nestedElements[*]> {
        POPPED_FIELD field2 <field1>
        FIELD field3 <field3>
        ITERATOR nestedIterator <nestedElements[*]> {
            POPPED_FIELD field2 <field1>
            FIELD field3 <field3>
        }
    }
}";

    let (tokens_opt, errors) =
        lexer::iterators().then(end()).parse_recovery(iter_str);
    assert!(errors.is_empty(), "{:?}", errors);

    let inner_fields = vec![
        Field {
            field_type: FieldType::Pop,
            ident:      "field2".to_string(),
            query:      "field1".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field3".to_string(),
            query:      "field3".to_string(),
        },
    ];

    let innermost_iter = Iterator {
        ident:           "nestedIterator".to_string(),
        query:           Some("nestedElements[*]".to_string()),
        iter_type:       None,
        fields:          inner_fields,
        nested_iterator: vec![],
    };

    let inner_iter = Iterator {
        nested_iterator: vec![innermost_iter.clone()],
        ..innermost_iter
    };

    let fields = vec![Field {
        field_type: FieldType::Push,
        ident:      "field1".to_string(),
        query:      "id".to_string(),
    }];

    let expected_items = Some(vec![Iterator {
        ident: "example".to_string(),
        query: Some("$".to_string()),
        iter_type: Some("jsonpath:".parse().unwrap()),
        fields,
        nested_iterator: vec![inner_iter],
    }]);

    let (parsed_items, errors) =
        parser::iterators().parse_recovery(tokens_opt.unwrap().0);

    assert!(errors.len() == 0, "{:?}", errors);
    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn iterator_nested_same_level_test() {
    let iter_str = "
    ITERATOR example <jsonpath: $> {
    PUSHED_FIELD field1 <id>
    ITERATOR nestedIterator <nestedElements[*]> {
        POPPED_FIELD field2 <field1>
        FIELD field3 <field3>
        ITERATOR nestedIterator <nestedElements[*]> {
            POPPED_FIELD field2 <field1>
            FIELD field3 <field3>
        }
    }
        ITERATOR nestedIterator <nestedElements[*]> {
            POPPED_FIELD field2 <field1>
            FIELD field3 <field3>
        }
}";

    let (tokens_opt, errors) =
        lexer::iterators().then(end()).parse_recovery(iter_str);
    assert!(errors.is_empty(), "{:?}", errors);

    let inner_fields = vec![
        Field {
            field_type: FieldType::Pop,
            ident:      "field2".to_string(),
            query:      "field1".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field3".to_string(),
            query:      "field3".to_string(),
        },
    ];
    let inner_most = Iterator {
        ident:           "nestedIterator".to_string(),
        query:           Some("nestedElements[*]".to_string()),
        iter_type:       None,
        fields:          inner_fields,
        nested_iterator: vec![],
    };

    let inner_iter1 = Iterator {
        nested_iterator: vec![inner_most.clone()],
        ..inner_most.clone()
    };

    let inner_iter2 = Iterator { ..inner_most };

    let fields = vec![Field {
        field_type: FieldType::Push,
        ident:      "field1".to_string(),
        query:      "id".to_string(),
    }];

    let expected_items = Some(vec![Iterator {
        ident: "example".to_string(),
        query: Some("$".to_string()),
        iter_type: Some("jsonpath:".parse().unwrap()),
        fields,
        nested_iterator: vec![inner_iter1, inner_iter2],
    }]);

    let (parsed_items, errors) =
        parser::iterators().parse_recovery(tokens_opt.unwrap().0);

    assert!(errors.len() == 0, "{:?}", errors);
    assert_parse_expected(parsed_items, expected_items)
}
#[test]
fn iterator_test() {
    let iter_str = "
ITERATOR example <xpath: /path/to/entity> {
    FIELD field1 <@attribute>
    FIELD field2 <field2>
    FIELD field3 <path/to/field3>
}";

    let (tokens_opt, errors) = lexer::iterators()
        .then_ignore(end())
        .parse_recovery(iter_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let parsed_items = parser::iterators().parse(tokens_opt.unwrap()).ok();

    let fields = vec![
        Field {
            field_type: FieldType::Normal,
            ident:      "field1".to_string(),
            query:      "@attribute".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field2".to_string(),
            query:      "field2".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field3".to_string(),
            query:      "path/to/field3".to_string(),
        },
    ];

    let expected_items = Some(vec![Iterator {
        ident: "example".to_string(),
        query: Some("/path/to/entity".to_string()),
        iter_type: Some("xpath:".parse().unwrap()),
        fields,
        nested_iterator: vec![],
    }]);
    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn iterator_csvperrrow_test() {
    let iter_str = "
ITERATOR example <csvperrow> {
    FIELD field1 <@attribute>
    FIELD field2 <field2>
    FIELD field3 <path/to/field3>
}";

    let (tokens_opt, errors) = lexer::iterators()
        .then_ignore(end())
        .parse_recovery(iter_str);

    assert!(errors.len() == 0, "{:?}", errors);
    let parsed_items = parser::iterators().parse(tokens_opt.unwrap()).ok();

    let fields = vec![
        Field {
            field_type: FieldType::Normal,
            ident:      "field1".to_string(),
            query:      "@attribute".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field2".to_string(),
            query:      "field2".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field3".to_string(),
            query:      "path/to/field3".to_string(),
        },
    ];

    let expected_items = Some(vec![Iterator {
        ident: "example".to_string(),
        query: None,
        iter_type: Some("csvperrow".parse().unwrap()),
        fields,
        nested_iterator: vec![],
    }]);
    assert_parse_expected(parsed_items, expected_items)
}
#[test]
fn prefix_multiple_test() {
    let prefix_1 = r#"PREFIX ex: <https://example.com/>
    PREFIX ex23: <https://example23.com/>

        "#;

    let (tokens_opt, _) = lexer::prefixes().parse_recovery(prefix_1);
    println!("{:?}", tokens_opt);
    let (parsed_items, error) =
        parser::prefixes().parse_recovery(tokens_opt.unwrap());

    assert!(error.len() == 0, "{:#?}", error);
    let expected_items = Some(vec![
        Prefix {
            prefix: PrefixNameSpace::NamedPrefix("ex".to_string()),
            uri:    "https://example.com/".to_string(),
        },
        Prefix {
            prefix: PrefixNameSpace::NamedPrefix("ex23".to_string()),
            uri:    "https://example23.com/".to_string(),
        },
    ]);
    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn prefix_test() {
    let prefix_1 = "PREFIX ex: <https://example.com/>";

    let (tokens_opt, _) = lexer::prefixes().parse_recovery(prefix_1);
    println!("{:?}", tokens_opt);
    let (parsed_items, error) =
        parser::prefixes().parse_recovery(tokens_opt.unwrap());

    assert!(error.len() == 0, "{:#?}", error);
    let expected_items = Some(vec![Prefix {
        prefix: PrefixNameSpace::NamedPrefix("ex".to_string()),
        uri:    "https://example.com/".to_string(),
    }]);

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn source_multiple_test() {
    let source_str = r#"SOURCE xml_file <https://example.com/file.xml>
    SOURCE json_file <local/file.json>

        "#;
    let (tokens_opt, _) = lexer::sources().parse_recovery(source_str);
    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::sources().parse_recovery(tokens_opt.clone().unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_items = Some(vec![
        Source {
            ident:       "xml_file".to_string(),
            uri:         "https://example.com/file.xml".to_string(),
            source_type: SourceType::HTTPS,
        },
        Source {
            ident:       "json_file".to_string(),
            uri:         "local/file.json".to_string(),
            source_type: SourceType::File,
        },
    ]);
    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn source_test() {
    let source_str = "SOURCE xml_file <https://example.com/file.xml>";
    let (tokens_opt, _) = lexer::sources().parse_recovery(source_str);
    let (parsed_items, errors) =
        parser::sources().parse_recovery(tokens_opt.clone().unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_items = Some(vec![Source {
        ident:       "xml_file".to_string(),
        uri:         "https://example.com/file.xml".to_string(),
        source_type: SourceType::HTTPS,
    }]);
    assert_parse_expected(parsed_items, expected_items)
}
