use super::test_core::{Expect, Expectation};
use super::test_extractors::*;
use crate::new_rml::rml_model::v2::core::TriplesMap;

macro_rules! expect {
    ($extractor:expr, $expected:expr, $name:expr) => {
        Box::new(Expect {
            extractor: $extractor,
            expected:  $expected,
            name:      $name.to_string(),
        })
    };
}

pub fn expect_triples_map_base_iri(expected: &str) -> Box<dyn Expectation> {
    expect!(
        extract_base_iri,
        expected.to_string(),
        "TriplesMap's 
        base IRI"
    )
}
pub fn expect_triples_map_identifier(expected: &str) -> Box<dyn Expectation> {
    expect!(
        extract_triples_map_identifier,
        expected.to_string(),
        "TriplesMap Identifier"
    )
}

pub fn expect_predicate_object_maps_count(
    expected: usize,
) -> Box<dyn Expectation> {
    expect!(
        extract_predicate_object_maps_count,
        expected,
        "Predicate-Object Maps Count"
    )
}

pub fn expect_logical_source_iterator(expected: &str) -> Box<dyn Expectation> {
    expect!(
        extract_logical_source_iterator,
        expected.to_string(),
        "Logical Source Iterator"
    )
}

pub fn expect_logical_source_reference_formulation(
    expected: &str,
) -> Box<dyn Expectation> {
    expect!(
        extract_logical_source_reference_formulation,
        expected.to_string(),
        "Logical Source Reference Formulation"
    )
}

pub fn expect_subject_map_template(expected: &str) -> Box<dyn Expectation> {
    expect!(
        extract_subject_map_template,
        expected.to_string(),
        "Subject Map Template"
    )
}

pub fn expect_subject_map_has_classes(expected: bool) -> Box<dyn Expectation> {
    expect!(
        extract_subject_map_has_classes,
        expected,
        "Subject Map Has Classes"
    )
}

pub fn expect_subject_map_term_type(expected: &str) -> Box<dyn Expectation> {
    expect!(
        extract_subject_map_term_type,
        expected.to_string(),
        "Subject Map Term Type"
    )
}

pub fn expect_subject_map_first_class(expected: &str) -> Box<dyn Expectation> {
    expect!(
        extract_subject_map_first_class,
        expected.to_string(),
        "Subject Map First Class"
    )
}

// Source expectations
pub fn expect_source_path(expected: &str) -> Box<dyn Expectation> {
    expect!(extract_source_path, expected.to_string(), "Source Path")
}

pub fn expect_source_root(expected: &str) -> Box<dyn Expectation> {
    expect!(extract_source_root, expected.to_string(), "Source Root")
}

pub fn expect_source_type(expected: &str) -> Box<dyn Expectation> {
    expect!(extract_source_type, expected.to_string(), "Source Type")
}

// Generic functions that take POM index as parameter
pub fn expect_predicate_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Predicate from POM {}", index);

    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_predicate_from_pom(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_object_reference_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Object Reference from POM {}", index);

    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_object_reference_from_pom(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_object_template_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Object Template from POM {}", index);

    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_object_template_from_pom(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_object_term_type_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Object Term Type from POM {}", index);

    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_object_term_type_from_pom(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_has_ref_object_map_in_pom(
    index: usize,
    expected: bool,
) -> Box<dyn Expectation> {
    let name = format!("Has RefObjectMap in POM {}", index);
    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<bool, String> {
            extract_has_ref_object_map_in_pom(triplesmap, index)
        },
        expected,
        name,
    })
}

pub fn expect_ref_object_map_parent_triples_map_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("RefObjectMap Parent TriplesMap from POM {}", index);
    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_ref_object_map_parent_triples_map(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_join_condition_child_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Join Condition Child from POM {}", index);
    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_join_condition_child(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_join_condition_parent_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Join Condition Parent from POM {}", index);
    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_join_condition_parent(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_subject_map_contains_class(
    class_uri: &str,
) -> Box<dyn Expectation> {
    let class_uri_owned = class_uri.to_string();
    let name = format!("Subject Map Contains {}", class_uri);

    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<bool, String> {
            let classes = extract_subject_map_classes(triplesmap)?;
            Ok(classes.contains(&class_uri_owned))
        },
        expected: true,
        name,
    })
}

pub fn expect_object_constant_from_pom(
    index: usize,
    expected: &str,
) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = format!("Object Constant from POM {}", index);

    Box::new(Expect {
        extractor: move |triplesmap: &TriplesMap| -> Result<String, String> {
            extract_object_constant_from_pom(triplesmap, index)
        },
        expected: expected_str,
        name,
    })
}

pub fn expect_subject_map_constant(expected: &str) -> Box<dyn Expectation> {
    let expected_str = expected.to_string();
    let name = "Subject Map Constant".to_string();

    Box::new(Expect {
        extractor: extract_subject_map_constant,
        expected: expected_str,
        name,
    })
}
