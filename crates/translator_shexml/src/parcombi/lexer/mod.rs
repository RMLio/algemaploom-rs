mod tests;
pub mod token;

use chumsky::chain::Chain;
use chumsky::prelude::*;
use chumsky::Parser;

use self::token::ShExMLToken;

macro_rules! t {
    ($t:ty) => {
        impl Parser<char, $t, Error = Simple<char>> + Clone
    };
}

macro_rules! shape_node {
    ($t:ident) => {
        pn_prefix()
            .or_not()
            .then_ignore(just(':'))
            .then(pn_char().repeated().at_least(1))
            .map(|(prefix_opt, local)| {
                let mut prefix: String = "".to_string();
                let local = local.into_iter().collect();
                if let Some(found_prefix) = prefix_opt {
                    prefix = found_prefix.into_iter().collect();
                }
                ShExMLToken::$t { prefix, local }
            })
    };
}

fn token(st: &'static str, token: ShExMLToken) -> t!(ShExMLToken) {
    just(st).padded().to(token)
}

fn within_angled_brackets() -> t!(String) {
    none_of("<>")
        .repeated()
        .at_least(1)
        .padded()
        .map(|c| c.into_iter().collect::<String>())
}

pub fn shexml() -> t!(Vec<ShExMLToken>) {
    prefixes()
        .chain::<ShExMLToken, _, _>(sources())
        .chain::<ShExMLToken, _, _>(iterators())
        .chain::<ShExMLToken, _, _>(expressions())
        .chain::<ShExMLToken, _, _>(shapes())
}

pub fn shapes() -> t!(Vec<ShExMLToken>) {
    let shape_node = shape_node!(ShapeNode);
    let predicate = shape_node!(ShapeTerm).or(token("a ", ShExMLToken::Type));
    let graph_node = shape_node!(ShapeNode);

    let pred_object = predicate
        .padded()
        .chain(shape_object())
        .padded()
        .chain(token(";", ShExMLToken::PredicateSplit));

    let subject = prefix_namespace()
        .chain::<ShExMLToken, _, _>(shape_node_expression())
        .or(shape_node!(ShapeTerm).map(|term| vec![term]));

    let single_shape = shape_node
        .padded()
        .chain(subject)
        .padded()
        .chain(token("{", ShExMLToken::CurlStart))
        .padded()
        .chain::<ShExMLToken, _, _>(
            pred_object.repeated().at_least(1).flatten(),
        )
        .padded()
        .chain(token("}", ShExMLToken::CurlEnd))
        .padded();

    let with_graph = graph_node
        .chain(
            token("[", ShExMLToken::SqBrackStart)
                .repeated()
                .at_least(2)
                .at_most(2),
        )
        .chain::<ShExMLToken, _, _>(
            single_shape.clone().repeated().at_least(1).flatten(),
        )
        .padded()
        .chain::<ShExMLToken, _, _>(
            token("]", ShExMLToken::SqBrackEnd)
                .repeated()
                .at_least(2)
                .at_most(2),
        );

    (with_graph.or(single_shape))
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
        .labelled("lexer:shapes")
}

fn shape_object() -> t!(Vec<ShExMLToken>) {
    let language_tag = token("@", ShExMLToken::AtSymb).chain(
        shape_node_expression().or(pn_char().repeated().at_least(1).map(
            |chars| vec![ShExMLToken::LangTag(chars.into_iter().collect())],
        )),
    );

    let data_type_static = prefix_namespace().chain::<ShExMLToken, _, _>(
        pn_char().repeated().at_least(1).map(|chars| {
            vec![ShExMLToken::PrefixLN(chars.into_iter().collect())]
        }),
    );

    let data_type = choice((
        prefix_namespace().chain(shape_node_expression()),
        shape_node_expression(),
        data_type_static.clone(),
    ));

    let shape_link =
        token("@", ShExMLToken::AtSymb).chain(shape_node!(ShapeNode));
    let object_literal = shape_node!(ShapeTerm).map(|t| vec![t]);

    let object = choice((
        shape_link,
        object_literal,
        prefix_namespace().chain::<ShExMLToken, _, _>(shape_node_expression()),
        shape_node_expression(),
    ))
    .padded();

    object
        .padded()
        .chain::<ShExMLToken, _, _>(language_tag.or(data_type).or_not())
        .padded()
        .labelled("lexer:shape_object")
}

fn shape_node_expression() -> t!(Vec<ShExMLToken>) {
    let matching = shape_sub_ident()
        .padded()
        .chain(token("MATCHING", ShExMLToken::Matching))
        .chain(ident());

    let if_block = shape_sub_ident()
        .padded()
        .chain(token("IF", ShExMLToken::If))
        .chain::<ShExMLToken, _, _>(shape_function_application());

    token("[", ShExMLToken::SqBrackStart)
        .chain(choice((
            matching,
            if_block,
            shape_function_application(),
            shape_sub_ident(),
        )))
        .chain(token("]", ShExMLToken::SqBrackEnd))
        .labelled("lexer:shape_node_expression")
}

fn shape_sub_ident() -> t!(Vec<ShExMLToken>) {
    ident()
        .chain(
            token(".", ShExMLToken::Dot)
                .chain(ident())
                .repeated()
                .flatten(),
        )
        .labelled("lexer:shape_sub_ident")
}

fn shape_function_application() -> t!(Vec<ShExMLToken>) {
    shape_sub_ident()
        .chain(token("(", ShExMLToken::BrackStart))
        .chain::<ShExMLToken, _, _>(shape_sub_ident())
        .chain::<ShExMLToken, _, _>(
            token(",", ShExMLToken::Comma)
                .padded()
                .chain(shape_sub_ident())
                .repeated()
                .flatten(),
        )
        .chain(token(")", ShExMLToken::BrackEnd))
        .labelled("lexer:shape_function_application")
}

pub fn expressions() -> t!(Vec<ShExMLToken>) {
    expression_stmt()
        .or(matcher())
        .or(autoincrement())
        .or(function())
        .padded()
        .repeated()
        .flatten()
}

pub fn function() -> t!(Vec<ShExMLToken>) {
    let function_tag = token("FUNCTIONS", ShExMLToken::Function).padded();
    let function_ident = ident().padded();

    let protocol = protocol().padded().map(ShExMLToken::FunctionLang);
    let uri = protocol_iri_ref().or(path().map(ShExMLToken::URI));

    let function_exp = token("<", ShExMLToken::AngleStart)
        .padded()
        .chain(protocol.chain(uri))
        .chain(token(">", ShExMLToken::AngleEnd).padded());

    (function_tag.chain(function_ident).chain(function_exp))
        .padded()
        .labelled("lexer:function")
}

pub fn autoincrement() -> t!(Vec<ShExMLToken>) {
    let aut_inc_tag =
        token("AUTOINCREMENT", ShExMLToken::AutoIncrement).padded();
    let ident = ident().padded();
    let prefix_str = text::ident::<char, _>()
        .map(|chars: String| ShExMLToken::AutoIncPrefix(chars))
        .delimited_by(just('"'), just('"'))
        .then_ignore(just('+').padded());

    let sufix_str = just('+').padded().ignore_then(
        text::ident()
            .map(|chars: String| ShExMLToken::AutoIncSuffix(chars))
            .delimited_by(just('"'), just('"')),
    );

    let start_inc = text::digits::<char, _>(10)
        .padded()
        .map(|digit: String| ShExMLToken::AutoIncStart(digit.parse().unwrap()));

    let end_inc =
        just("to").padded().ignore_then(text::digits(10).map(
            |digit: String| ShExMLToken::AutoIncEnd(digit.parse().unwrap()),
        ));

    let step_inc =
        just("by").padded().ignore_then(text::digits(10).map(
            |digit: String| ShExMLToken::AutoIncStep(digit.parse().unwrap()),
        ));

    let aut_inc_exp = prefix_str
        .or_not()
        .chain::<ShExMLToken, _, _>(start_inc)
        .chain::<ShExMLToken, _, _>(end_inc.or_not())
        .chain::<ShExMLToken, _, _>(step_inc.or_not())
        .chain::<ShExMLToken, _, _>(sufix_str.or_not());

    let auto_inc_exp_delim = token("<", ShExMLToken::AngleStart)
        .chain::<ShExMLToken, Vec<_>, _>(aut_inc_exp)
        .chain(token(">", ShExMLToken::AngleEnd));

    (aut_inc_tag.chain(ident).chain(auto_inc_exp_delim))
        .padded()
        .labelled("lexer:autoincrement")
}

pub fn matcher() -> t!(Vec<ShExMLToken>) {
    let mat_tag = token("MATCHER", ShExMLToken::Matcher);
    let mat_ident = ident().padded();
    let mats_value = none_of("<>,&")
        .repeated()
        .at_least(1)
        .padded()
        .map(|v_char: Vec<char>| v_char.into_iter().collect::<String>())
        .map(|str| {
            str.split(" AS ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .map(|mut splitted_str| {
            let mut result = vec![];

            if splitted_str.len() > 1 {
                let ident = ShExMLToken::Ident(
                    splitted_str.pop().unwrap().trim().to_string(),
                );
                let value = ShExMLToken::Value(
                    splitted_str.pop().unwrap().trim().to_string(),
                );
                result.push(value);
                result.push(ShExMLToken::As);
                result.push(ident);
            } else {
                let value = splitted_str.pop().unwrap();
                result.push(ShExMLToken::Value(value.trim().to_string()));
            }
            result
        });

    (mat_tag.chain(mat_ident).chain(
        token("<", ShExMLToken::AngleStart)
            .chain(
                mats_value
                    .then(
                        token(",", ShExMLToken::Comma)
                            .or(token("&", ShExMLToken::MatcherSplit))
                            .or_not(),
                    )
                    .map(|(mut tokens, opt_tok)| {
                        if let Some(delim_tok) = opt_tok {
                            tokens.push(delim_tok);
                        }

                        tokens
                    })
                    .repeated()
                    .at_least(1)
                    .flatten(),
            )
            .chain(token(">", ShExMLToken::AngleEnd)),
    ))
    .padded()
    .labelled("lexer:matcher")
}

pub fn expression_stmt() -> t!(Vec<ShExMLToken>) {
    let expressiont_tag = token("EXPRESSION", ShExMLToken::Expression);
    let exp_ident = ident().padded();

    let sub_ident = ident()
        .chain(just('.').to(ShExMLToken::Dot))
        .repeated()
        .at_least(1)
        .flatten()
        .chain(ident());

    let str_op_right = just('+')
        .padded()
        .ignored()
        .then(
            pn_char()
                .repeated()
                .at_least(1)
                .delimited_by(just('"'), just('"'))
                .map(|string_sep| {
                    ShExMLToken::StringSep(string_sep.into_iter().collect())
                })
                .then_ignore(just('+').padded()),
        )
        .then(sub_ident.clone().padded())
        .map(|((_, token), tokens)| {
            let mut result = vec![token];

            result.extend_from_slice(&tokens);
            result
        });

    let str_operation = sub_ident
        .clone()
        .chain(str_op_right.repeated().at_least(1).flatten())
        .labelled("lexer:string_op");

    let basic_expression = str_operation
        .clone()
        .or(sub_ident.clone())
        .labelled("lexer:basic_expression");

    let union_tok = token("UNION", ShExMLToken::Union);
    let join_tok = token("JOIN", ShExMLToken::Join);

    let join = basic_expression
        .clone()
        .chain(union_tok.clone())
        .chain::<ShExMLToken, _, _>(basic_expression.clone())
        .chain(join_tok.clone())
        .chain(basic_expression.clone())
        .padded()
        .labelled("lexer:join");

    let union = basic_expression
        .clone()
        .chain(union_tok.clone())
        .repeated()
        .flatten()
        .chain(basic_expression.clone())
        .padded()
        .labelled("lexer:union");

    let exp_inner = token("<", ShExMLToken::AngleStart)
        .chain(
            join.or(union)
                .or(str_operation)
                .or(sub_ident.clone())
                .repeated()
                .at_least(1)
                .flatten(),
        )
        .chain(token(">", ShExMLToken::AngleEnd));
    (expressiont_tag.chain(exp_ident).chain(exp_inner))
        .padded()
        .labelled("lexer:expression_stmt")
}

pub fn iterators() -> t!(Vec<ShExMLToken>) {
    let header = iterator_header().padded();

    (recursive(|recur| {
        header
            .chain(token("{", ShExMLToken::BrackStart))
            .chain::<ShExMLToken, _, _>(
                field().repeated().at_least(1).flatten(),
            )
            .chain::<ShExMLToken, _, _>(
                token("}", ShExMLToken::BrackEnd)
                    .map(|tok| vec![tok])
                    .or(recur),
            )
            .chain(
                token("}", ShExMLToken::BrackEnd)
                    .map(|tok| vec![tok])
                    .or_not(),
            )
            .padded()
    }))
    .padded()
    .repeated()
    .at_least(1)
    .flatten()
    .labelled("lexer:iterators")
}

fn field() -> t!(Vec<ShExMLToken>) {
    let field_tag = token("FIELD", ShExMLToken::Field);
    let push_field_tag = token("PUSHED_FIELD", ShExMLToken::PushField);
    let pop_field_tag = token("POPPED_FIELD", ShExMLToken::PopField);
    let field_name = ident().padded();
    let field_query = within_angled_brackets()
        .padded()
        .delimited_by(just("<"), just(">"))
        .map(ShExMLToken::FieldQuery);

    choice((field_tag, push_field_tag, pop_field_tag))
        .chain(field_name)
        .chain(field_query)
        .labelled("lexer:field")
}

fn iterator_header() -> t!(Vec<ShExMLToken>) {
    let iterator_tag = token("ITERATOR", ShExMLToken::Iterator);
    let iterator_name = ident().padded();

    let iterator_type = protocol().padded().map(ShExMLToken::IteratorType);
    let csv_iterator_type = pn_char()
        .repeated()
        .at_least(1)
        .padded()
        .map(|chars| chars.into_iter().collect::<String>())
        .map(|iter_string| iter_string.to_lowercase())
        .try_map(|lowered_string, span| {
            if lowered_string == "csvperrow" {
                Ok(ShExMLToken::IteratorType(lowered_string))
            } else {
                Err(Simple::custom(
                    span,
                    format!("Not csvperrow iterator: {}", lowered_string),
                ))
            }
        });

    let iterator_query =
        within_angled_brackets().map(ShExMLToken::IteratorQuery);

    let iter_query_pair = token("<", ShExMLToken::AngleStart)
        .chain(
            iterator_type
                .or(csv_iterator_type)
                .or_not()
                .chain::<ShExMLToken, _, _>(iterator_query.or_not()),
        )
        .chain(token(">", ShExMLToken::AngleEnd));

    iterator_tag
        .chain(iterator_name)
        .chain(iter_query_pair)
        .repeated()
        .at_least(1)
        .flatten()
        .labelled("lexer:iterator_header")
}

pub fn sources() -> t!(Vec<ShExMLToken>) {
    let source_tag = token("SOURCE", ShExMLToken::Source);
    let source_name = ident().padded();

    let source_iri =
        token("<", ShExMLToken::AngleStart)
            .chain(protocol_iri_tokenize().or(
                path().map(|c| vec![ShExMLToken::File, ShExMLToken::URI(c)]),
            ))
            .chain(token(">", ShExMLToken::AngleEnd));
    source_tag
        .chain(source_name)
        .chain(source_iri)
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
        .labelled("lexer:sources")
}

fn ident() -> t!(ShExMLToken) {
    pn_char()
        .repeated()
        .at_least(1)
        .map(|v| ShExMLToken::Ident(v.into_iter().collect()))
        .labelled("lexer:ident")
}

pub fn prefixes() -> t!(Vec<ShExMLToken>) {
    let prefix_tag = token("PREFIX", ShExMLToken::Prefix);
    let pname = prefix_namespace();

    prefix_tag
        .chain(pname)
        .chain(
            token("<", ShExMLToken::AngleStart)
                .chain(protocol_iri_ref())
                .chain(token(">", ShExMLToken::AngleEnd)),
        )
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
        .labelled("lexer:prefixes")
}

fn prefix_namespace() -> t!(Vec<ShExMLToken>) {
    pn_prefix()
        .or(just("_").map(|_| vec!['_']))
        .or_not()
        .then_ignore(just(":"))
        .padded()
        .flatten()
        .map(|pname_vec| {
            if pname_vec.is_empty() {
                vec![ShExMLToken::BasePrefix, ShExMLToken::PrefixSep]
            } else {
                let prefix = pname_vec.into_iter().collect();
                vec![ShExMLToken::PrefixNS(prefix), ShExMLToken::PrefixSep]
            }
        })
        .labelled("lexer:prefix_namespace")
}

fn protocol() -> t!(String) {
    filter(|c: &char| c.is_ascii_alphabetic())
        .repeated()
        .at_least(1)
        .chain(just(':'))
        .map(|c| c.into_iter().collect())
        .labelled("lexer:protocol")
}

fn path() -> t!(String) {
    none_of("<>\"{}|^`\\[]")
        .repeated()
        .at_least(1)
        .map(|e| e.into_iter().collect())
}

fn protocol_iri_ref_new() -> t!(Vec<String>) {
    protocol()
        .repeated()
        .at_most(2)
        .chain::<String, _, _>(just("//").map(|c| c.to_string()))
        .chain(path())
}

fn protocol_iri_tokenize() -> t!(Vec<ShExMLToken>) {
    protocol_iri_ref_new().map(|vec_string| {
        if vec_string.len() == 1 {
            vec_string
                .into_iter()
                .map(|path| ShExMLToken::URI(path))
                .collect()
        } else {
            let mut result = Vec::new();
            let mut iter = vec_string.into_iter();
            let mut uri_string = String::new();

            while let Some(iri_part) = iter.next() {
                uri_string += &iri_part;
                let token = match iri_part.as_str() {
                    "http:" => ShExMLToken::HTTP,
                    "https:" => ShExMLToken::HTTPS,
                    "file:" => ShExMLToken::File,
                    "jdbc:" => {
                        let jdbc_type = iter.next().unwrap();
                        uri_string += &jdbc_type;
                        ShExMLToken::JDBC(jdbc_type)
                    }
                    _ => continue,
                };

                result.push(token)
            }

            result.push(ShExMLToken::URI(uri_string));

            result
        }
    })
}

fn protocol_iri_ref() -> t!(ShExMLToken) {
    let uri = protocol()
        .chain::<String, _, _>(just("//").map(|c| c.to_string()))
        .chain(path())
        .map(|z: Vec<String>| ShExMLToken::URI(z.into_iter().collect()));

    uri
}

fn pn_char_base() -> t!(char) {
    filter(|c: &char| c.is_alphabetic())
}

fn pn_char_u() -> t!(char) {
    pn_char_base().or(just('_'))
}
fn pn_char() -> t!(char) {
    pn_char_u()
        .or(just('-'))
        .or(filter(|c: &char| c.is_numeric()))
}
fn pn_prefix() -> t!(Vec<char>) {
    let ne = just('.')
        .repeated()
        .then(pn_char().repeated().at_least(1))
        .map(|(x, y)| {
            let mut o: Vec<char> = Vec::with_capacity(x.len() + y.len());
            x.append_to(&mut o);
            y.append_to(&mut o);
            o
        })
        .repeated()
        .flatten();

    pn_char_base().then(ne.or_not()).map(|(x, y)| {
        if let Some(y) = y {
            let mut o = Vec::with_capacity(y.len() + 1);
            o.push(x);
            o.extend(y);
            o
        } else {
            vec![x]
        }
    })
}
