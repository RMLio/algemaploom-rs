#[cfg(test)]
use super::*;

#[test]
fn protocol_test() {
    let proc1 = "jdbc:".to_string();
    let proc2 = "http:".to_string();
    let protocol_str = proc1.clone() + &proc2;

    let (tokens_opt, errors) = protocol()
        .repeated()
        .at_least(1)
        .parse_recovery(protocol_str);

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![proc1, proc2]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn shexml_test() {
    let shexml_doc = r#"

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
    :director [films.directors];
} "#;

    let (tokens_opt, errors) = shexml().parse_recovery(shexml_doc);

    println!("{:#?}", tokens_opt);

    assert!(errors.len() == 0, "{:?}", errors);
}

#[test]
fn function_if_shape_test() {
    let shape_str = "

:Films :[films.id IF helper.isBefore2010(films.year)] {
    :name [films.name] ;
    :year [films.year] ;
    :countryOfOrigin [films.country IF helper.outsideUSA(films.country)] ;
}        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());
    let helper = ShExMLToken::Ident("helper".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::If,
        helper.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("isBefore2010".to_string()),
        ShExMLToken::BrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::BrackEnd,
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "countryOfOrigin".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("country".to_string()),
        ShExMLToken::If,
        helper.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("outsideUSA".to_string()),
        ShExMLToken::BrackStart,
        ShExMLToken::Ident("films".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("country".to_string()),
        ShExMLToken::BrackEnd,
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn function_shape_test() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] ;
    :year :[films.year] ;
    :bigName dbr:[helper.allCapitals(films.name)] ;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "bigName".to_string(),
        },
        ShExMLToken::PrefixNS("dbr".to_string()),
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        ShExMLToken::Ident("helper".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("allCapitals".to_string()),
        ShExMLToken::BrackStart,
        ShExMLToken::Ident("films".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::BrackEnd,
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn matching_shape_test() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] ;
    :year :[films.year] ;
    :country dbr:[films.country MATCHING spain] ;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "country".to_string(),
        },
        ShExMLToken::PrefixNS("dbr".to_string()),
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("country".to_string()),
        ShExMLToken::Matching,
        ShExMLToken::Ident("spain".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn graphed_shape_test() {
    let shape_str = "

:MyGraph [[
    :Films :[films.id] {
        :name [films.name] ;
        :year :[films.year] ;
    }
]]
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "MyGraph".to_string(),
        },
        ShExMLToken::SqBrackStart,
        ShExMLToken::SqBrackStart,
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
        ShExMLToken::SqBrackEnd,
        ShExMLToken::SqBrackEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_matching() {
    let shape_str = "

:Films :[films.id] {
   :country dbr:[films.country MATCHING spain] ;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "country".to_string(),
        },
        ShExMLToken::PrefixNS("dbr".to_string()),
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        ShExMLToken::Ident("films".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("country".to_string()),
        ShExMLToken::Matching,
        ShExMLToken::Ident("spain".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_two_shape_linking_test() {
    let shape_str = "

:Films :[films.id] {
    :name @:Names;
}

:Names  :[films.name]{
    :review :good; 
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());
    let mut first_shape = vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::AtSymb,
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Names".to_string(),
        },
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ];

    let second_shape = vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Names".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "review".to_string(),
        },
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "good".to_string(),
        },
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ];
    first_shape.extend(second_shape);
    let expected = Some(first_shape);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_linking() {
    let shape_str = "

:Films :[films.id] {
    :name @:Names;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::AtSymb,
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Names".to_string(),
        },
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_object_literal() {
    let shape_str = "

:Films :[films.id] {
    :name ex:Jackie;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::ShapeTerm {
            prefix: "ex".to_string(),
            local:  "Jackie".to_string(),
        },
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_datatype_static() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] xsd:string;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PrefixNS("xsd".to_string()),
        ShExMLToken::PrefixSep,
        ShExMLToken::PrefixLN("string".to_string()),
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_datatype_dynamic() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] xsd:[films.nameDatatype];
    :year :[films.year] [films.yearDatatype] ;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PrefixNS("xsd".to_string()),
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("nameDatatype".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("yearDatatype".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_languagetag_test() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] @be-nl;
    :year :[films.year] @[films.yearlang] ;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::AtSymb,
        ShExMLToken::LangTag("be-nl".to_string()),
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::AtSymb,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("yearlang".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_subject_fixed_test() {
    let shape_str = "

:Films :film1 {
    a :Film;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "film1".to_string(),
        },
        ShExMLToken::CurlStart,
        ShExMLToken::Type,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "Film".to_string(),
        },
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_class_type_test() {
    let shape_str = "

:Films :[films.id] {
    a :Film;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::Type,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "Film".to_string(),
        },
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn simple_single_shape_test() {
    let shape_str = "

:Films :[films.id] {
    :name [films.name] ;
    :year :[films.year] ;
}
        ";

    let (tokens_opt, errors) = shapes()
        .padded()
        .then_ignore(end())
        .parse_recovery(shape_str);

    println!("{:#?}", tokens_opt);
    let film_exp = ShExMLToken::Ident("films".to_string());

    let expected = Some(vec![
        ShExMLToken::ShapeNode {
            prefix: "".to_string(),
            local:  "Films".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::CurlStart,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "name".to_string(),
        },
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::ShapeTerm {
            prefix: "".to_string(),
            local:  "year".to_string(),
        },
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::SqBrackStart,
        film_exp.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("year".to_string()),
        ShExMLToken::SqBrackEnd,
        ShExMLToken::PredicateSplit,
        ShExMLToken::CurlEnd,
    ]);

    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn function_test() {
    let function_str = "
        FUNCTIONS helper <scala: https://raw.githubusercontent.com/herminiogg/ShExML/enhancement-%23121/src/test/resources/functions.scala>
        ";
    let (tokens_opt, errors) = function()
        .padded()
        .then_ignore(end())
        .parse_recovery(function_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::Function,
        ShExMLToken::Ident("helper".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::FunctionLang("scala:".to_string()),
        ShExMLToken::URI("https://raw.githubusercontent.com/herminiogg/ShExML/enhancement-%23121/src/test/resources/functions.scala".to_string()),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt
    )
}

#[test]
fn auto_inc_only_start_test() {
    let match_str = "
     AUTOINCREMENT myId <2>   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncStart(2),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt,
    )
}

#[test]
fn auto_inc_end_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 to 20>   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncPrefix("my".to_string()),
        ShExMLToken::AutoIncStart(0),
        ShExMLToken::AutoIncEnd(20),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt,
    )
}

#[test]
fn auto_inc_start_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 >   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncPrefix("my".to_string()),
        ShExMLToken::AutoIncStart(0),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt,
    )
}

#[test]
fn auto_inc_complete_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 to 10 by 2 + \"Id\">   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncPrefix("my".to_string()),
        ShExMLToken::AutoIncStart(0),
        ShExMLToken::AutoIncEnd(10),
        ShExMLToken::AutoIncStep(2),
        ShExMLToken::AutoIncSuffix("Id".to_string()),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        expected,
        tokens_opt,
    )
}

#[test]
fn multiple_matching_matcher_test() {
    let match_str = "
        MATCHER regions <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias &
                Spain, España, Espagne AS Spain>
        ";

    let (tokens_opt, errors) = matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
}

#[test]
fn single_matcher_test() {
    let match_str = "
        MATCHER ast <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias>
        ";

    let (tokens_opt, errors) = matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    let expected_tokens = Some(vec![
        ShExMLToken::Matcher,
        ShExMLToken::Ident("ast".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::Value("Principality of Asturias".to_string()),
        ShExMLToken::Comma,
        ShExMLToken::Value("Principado de Asturias".to_string()),
        ShExMLToken::Comma,
        ShExMLToken::Value("Principáu d'Asturies".to_string()),
        ShExMLToken::Comma,
        ShExMLToken::Value("Asturies".to_string()),
        ShExMLToken::As,
        ShExMLToken::Ident("Asturias".to_string()),
        ShExMLToken::AngleEnd,
    ]);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);

    assert!(
        expected_tokens == tokens_opt,
        "Expected tokens: {:#?}\nGenerated output: {:#?}",
        expected_tokens,
        tokens_opt
    );
}

// TODO: Lex string concatenation + union operation properly <12-03-24, yourname> //
#[test]
fn string_op_union_expression_test() {
    let exp_str = "EXPRESSION exp <file.it1.id + \"-seper-\" +  file.it2.name  UNION file.it3.union>";

    let (tokens_opt, errors) = expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery_verbose(exp_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);

    let file_ident = ShExMLToken::Ident("file".to_string());
    let expected_tokens = Some(vec![
        ShExMLToken::Expression,
        ShExMLToken::Ident("exp".to_string()),
        ShExMLToken::AngleStart,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it1".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::StringSep("-seper-".to_string()),
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it2".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::Union,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it3".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("union".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "Expected tokens: {:#?}, Generated Output: {:#?}",
        expected_tokens,
        tokens_opt
    );
}

#[test]
fn string_op_expression_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id + \"-seper-\" +  file.it2.name>
        ";

    let (tokens_opt, errors) = expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);

    let file_ident = ShExMLToken::Ident("file".to_string());
    let expected_tokens = Some(vec![
        ShExMLToken::Expression,
        ShExMLToken::Ident("exp".to_string()),
        ShExMLToken::AngleStart,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it1".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::StringSep("-seper-".to_string()),
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it2".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "Expected tokens: {:#?}, Generated Output: {:#?}",
        expected_tokens,
        tokens_opt
    );
}

#[test]
fn iterator_expression_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id>
        ";

    let (tokens_opt, errors) = expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);
    let file_ident = ShExMLToken::Ident("file".to_string());

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Expression,
        ShExMLToken::Ident("exp".to_string()),
        ShExMLToken::AngleStart,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it1".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    println!("{:?}", tokens_opt);

    assert!(
        tokens_opt == expected_tokens,
        "Expected tokens: {:#?}, Generated Output: {:#?}",
        expected_tokens,
        tokens_opt
    );
}

#[test]
fn join_union_expression_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id UNION file.it2.name JOIN file.it1.name>
        ";

    let (tokens_opt, errors) = expression_stmt()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);
    let file_ident = ShExMLToken::Ident("file".to_string());

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Expression,
        ShExMLToken::Ident("exp".to_string()),
        ShExMLToken::AngleStart,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it1".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("id".to_string()),
        ShExMLToken::Union,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it2".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::Join,
        file_ident.clone(),
        ShExMLToken::Dot,
        ShExMLToken::Ident("it1".to_string()),
        ShExMLToken::Dot,
        ShExMLToken::Ident("name".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    println!("{:?}", tokens_opt);

    assert!(
        tokens_opt == expected_tokens,
        "Expected tokens: {:#?}, Generated Output: {:#?}",
        expected_tokens,
        tokens_opt
    );
}

#[test]
fn iterator_test() {
    let iter_str = "
ITERATOR example <xpath: /path/to/entity> {
    FIELD field1 <@attribute>
    FIELD field2 <field2>
    FIELD field3 <path/to/field3>
}";

    let (tokens_opt, errors) = iterators().then(end()).parse_recovery(iter_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
}

#[test]
fn iterator_nest_test() {
    let iter_str = "
    ITERATOR example <jsonpath: $> {
    PUSHED_FIELD field1 <id>
    ITERATOR nestedIterator < nestedElements[*]> {
        POPPED_FIELD field2 <field1>
        FIELD field3 <field3>
        ITERATOR nestedIterator < nestedElements[*]> {
            POPPED_FIELD field2 <field1>
            FIELD field3 <field3>
        }
    }
}";

    let (tokens_opt, errors) = iterators().then(end()).parse_recovery(iter_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
}

#[test]
fn iterator_header_test() {
    let iter_str = "ITERATOR example <xpath: /path/to/entity>";
    let (tokens_opt, errors) = iterator_header()
        .padded()
        .then_ignore(end())
        .parse_recovery(iter_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Iterator,
        ShExMLToken::Ident("example".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::IteratorType("xpath:".to_string()),
        ShExMLToken::IteratorQuery("/path/to/entity".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn iterator_csvperrow_test() {
    let iter_str = "ITERATOR example <csvperrow>";
    let (tokens_opt, errors) = iterator_header()
        .padded()
        .then_ignore(end())
        .parse_recovery(iter_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Iterator,
        ShExMLToken::Ident("example".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::IteratorType("csvperrow".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn source_jdbc_test() {
    let source_str =
        "SOURCE sparql_endpoint <jdbc:sparql://localhost:6000/sparql/>";
    let (tokens_opt, errors) = sources().parse_recovery(source_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Source,
        ShExMLToken::Ident("sparql_endpoint".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::JDBC("sparql:".to_string()),
        ShExMLToken::URI("jdbc:sparql://localhost:6000/sparql/".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn source_local_path_test() {
    let source_str = "SOURCE json_file <file.json>";
    let (tokens_opt, errors) = sources().parse_recovery(source_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Source,
        ShExMLToken::Ident("json_file".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::File,
        ShExMLToken::URI("file.json".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn source_test() {
    let source_str = "SOURCE xml_file <https://example.com/file.xml>";
    let (tokens_opt, errors) = sources().parse_recovery(source_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Source,
        ShExMLToken::Ident("xml_file".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::HTTPS,
        ShExMLToken::URI("https://example.com/file.xml".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn empty_prefix_name_test() {
    let base_prefix = "PREFIX : <https://base.com/>";

    let (tokens_opt, errors) = prefixes().parse_recovery(base_prefix);

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Prefix,
        ShExMLToken::BasePrefix,
        ShExMLToken::PrefixSep,
        ShExMLToken::AngleStart,
        ShExMLToken::URI("https://base.com/".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn prefix_test() {
    let prefix_1 = "PREFIX ex: <https://example.com/>";

    let (tokens_opt, errors) = prefixes().parse_recovery(prefix_1);

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Prefix,
        ShExMLToken::PrefixNS("ex".to_string()),
        ShExMLToken::PrefixSep,
        ShExMLToken::AngleStart,
        ShExMLToken::URI("https://example.com/".to_string()),
        ShExMLToken::AngleEnd,
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}
