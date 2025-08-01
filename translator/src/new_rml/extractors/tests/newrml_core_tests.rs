use super::framework::test_core::test_triplesmaps;
use super::framework::test_expectations::*;
use crate::new_rml::extractors::tests::framework::test_core::expect_parse_fail;

#[cfg(test)]    
mod rml_core_tests {
    use super::*;

    #[test]
    fn test_rmltc0000_json_mapping() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0000-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("http://example.com/{$.Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002a_json_mapping() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0002a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    // Add expectations here when needed
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0001a_subject_map_reference() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0001a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("http://example.com/{$.Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0001b_blank_node_term_type() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0001b-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("{$.Name}"),
                    expect_subject_map_term_type("http://w3id.org/rml/BlankNode"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002b_blank_node_with_template() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0002b-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("students{$.ID}"),
                    expect_subject_map_term_type("http://w3id.org/rml/BlankNode"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002e() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0002e-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student2.json"),
                    expect_predicate_from_pom(0, "http://example.com/id"),
                    expect_object_reference_from_pom(0, "$.IDs"),
                    expect_subject_map_template("http://example.com/{$.ID}/{$.Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002g() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0002g-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student2.json"),
                    expect_predicate_from_pom(0, "http://example.com/id"),
                    expect_object_reference_from_pom(0, "$.IDs"),
                    expect_subject_map_template("http://example.com/{$.ID}/{$.Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0003c_object_template_literal() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0003c-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_template_from_pom(0, "{$.FirstName} {$.LastName}"),
                    expect_object_term_type_from_pom(0, "http://w3id.org/rml/Literal"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("http://example.com/Student{$.ID}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0004a_multiple_triplesmaps() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0004a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Student"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student_sport.json"),
                    expect_subject_map_template("http://example.com/{$.Student}"),
                    expect_subject_map_contains_class("http://example.com/Student"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Sport"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student_sport.json"),
                    expect_subject_map_template("http://example.com/{$.Sport}"),
                    expect_subject_map_contains_class("http://example.com/Sport"),
                ]),
            ]
        );
    }

    #[test]
    fn test_rmltc0004b_subject_literal_term_type() {
        expect_parse_fail("rml-core-tests/RMLTC0004b-JSON.ttl");
    }

    #[test]
    fn test_rmltc0005a_subject_class() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0005a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://example.com/owes"),
                    expect_object_reference_from_pom(0, "$.amount"),
                    expect_logical_source_iterator("$.persons[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("ious.json"),
                    expect_subject_map_template("http://example.com/{$.fname};{$.lname}"),
                    expect_subject_map_contains_class("http://xmlns.com/foaf/0.1/Person"),
                ])
            ]
        );
    }


    #[test]
    fn test_rmltc0006a() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0006a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_from_pom(0, "http://example.com/description"),
                    expect_object_constant_from_pom(0, "Bad Student"),
                    expect_subject_map_constant("http://example.com/BadStudent"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0007b() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0007b-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_object_constant_from_pom(0, "http://xmlns.com/foaf/0.1/Person"),
                    expect_predicate_from_pom(1, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(1, "$.FirstName"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.FirstName}"),
                ])
            ]
        );
    }

    

    #[test]
    fn test_rmltc0007a_constant_object() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0007a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.FirstName}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0007c() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0007c-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://example.com/id"),
                    expect_object_reference_from_pom(0, "$.ID"),
                    expect_predicate_from_pom(1, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(1, "$.FirstName"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.FirstName}"),
                    expect_subject_map_contains_class("http://example.com/Student"),
                    expect_subject_map_contains_class("http://xmlns.com/foaf/0.1/Person"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0007d() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0007d-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_object_maps_count(4),
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_object_constant_from_pom(0, "http://xmlns.com/foaf/0.1/Person"),
                    expect_predicate_from_pom(1, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_object_constant_from_pom(1, "http://example.com/Student"),
                    expect_predicate_from_pom(2, "http://example.com/id"),
                    expect_object_reference_from_pom(2, "$.ID"),
                    expect_predicate_from_pom(3, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(3, "$.FirstName"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.FirstName}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0008a_multiple_predicate_object_maps() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0008a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(4), 
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_predicate_from_pom(1, "http://example.com/id"),
                    expect_object_reference_from_pom(1, "$.ID"),
                    expect_predicate_from_pom(2, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(2, "$.Name"),
                    expect_predicate_from_pom(3, "http://example.com/Sport"),
                    expect_object_reference_from_pom(3, "$.Sport"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.Name}"),
                ])
            ]
        );
    }


    #[test]
    fn test_rmltc0008b() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0008b-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_object_maps_count(4),
                    expect_predicate_from_pom(0, "http://example.com/Sport"),
                    expect_has_ref_object_map_in_pom(0, true),
                    expect_ref_object_map_parent_triples_map_from_pom(0, "http://example.com/base/TriplesMap2"),
                    expect_predicate_from_pom(1, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_object_constant_from_pom(1, "http://xmlns.com/foaf/0.1/Person"),
                    expect_predicate_from_pom(2, "http://example.com/id"),
                    expect_object_reference_from_pom(2, "$.ID"),
                    expect_predicate_from_pom(3, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(3, "$.Name"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.Name}"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_object_constant_from_pom(0, "http://example.com/activity/Sport"),
                    expect_subject_map_template("http://example.com/{$.Sport}"),
                ]),
            ]
        );
    }

        #[test]
    fn test_rmltc0008c() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0008c-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_from_pom(0, "http://example.com/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_subject_map_template("http://example.com/Student/{$.ID}/{$.Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0009a_join_condition() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0009a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(2), 
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_predicate_from_pom(1, "http://example.com/ontology/practises"),
                    expect_has_ref_object_map_in_pom(1, true),
                    expect_ref_object_map_parent_triples_map_from_pom(1, "http://example.com/base/TriplesMap2"),
                    expect_join_condition_child_from_pom(1, "$.Sport"),
                    expect_join_condition_parent_from_pom(1, "$.ID"),
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_subject_map_template("http://example.com/resource/student_{$.ID}"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/2000/01/rdf-schema#label"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.sports[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("sport.json"),
                    expect_subject_map_template("http://example.com/resource/sport_{$.ID}"),
                ]),
            ]
        );
    }

        #[test]
    fn test_rmltc0009b() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0009b-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.students[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("student.json"),
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_predicate_from_pom(1, "http://example.com/ontology/practises"),
                    expect_has_ref_object_map_in_pom(1, true),
                    expect_ref_object_map_parent_triples_map_from_pom(1, "http://example.com/base/TriplesMap2"),
                    expect_join_condition_child_from_pom(1, "$.Sport"),
                    expect_join_condition_parent_from_pom(1, "$.ID"),
                    expect_subject_map_template("http://example.com/resource/student_{$.ID}"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_logical_source_iterator("$.sports[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("sport.json"),
                    expect_predicate_from_pom(0, "http://www.w3.org/2000/01/rdf-schema#label"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_subject_map_template("http://example.com/resource/sport_{$.ID}"),
                ]),
            ]
        );
    }

    #[test]
    fn test_rmltc0010a_bracket_reference() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0010a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://example.com/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.countries[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("country_info.json"),
                    expect_subject_map_template("http://example.com/{$.['Country Code']}"),
                ])
            ]
        );
    }

        #[test]
    fn test_rmltc0010b() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0010b-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_logical_source_iterator("$.countries[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("country_info.json"),
                    expect_predicate_from_pom(0, "http://example.com/name"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_subject_map_template("http://example.com/{$.['Country Code']}/{$.Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0012a_multiple_templates() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0012a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_template_from_pom(0, "{$.fname} {$.lname}"),
                    expect_object_term_type_from_pom(0, "http://w3id.org/rml/Literal"),
                    expect_logical_source_iterator("$.persons[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("persons.json"),
                    expect_subject_map_template("{$.fname}{$.lname}{$.amount}"),
                    expect_subject_map_term_type("http://w3id.org/rml/BlankNode"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0015a_language_tags() {
        test_triplesmaps(
            "rml-core-tests/RMLTC0015a-JSON.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/2000/01/rdf-schema#label"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.countries[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("country_en.json"),
                    expect_subject_map_template("http://example.com/{$.Code}"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/2000/01/rdf-schema#label"),
                    expect_object_reference_from_pom(0, "$.Name"),
                    expect_logical_source_iterator("$.countries[*]"),
                    expect_logical_source_reference_formulation("http://w3id.org/rml/JSONPath"),
                    expect_source_type("http://w3id.org/rml/RelativePathSource"),
                    expect_source_root("http://w3id.org/rml/MappingDirectory"),
                    expect_source_path("country_es.json"),
                    expect_subject_map_template("http://example.com/{$.Code}"),
                ]),
            ]
        );
    }












}