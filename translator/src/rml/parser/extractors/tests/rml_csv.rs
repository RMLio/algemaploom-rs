use super::framework::test_core::test_triplesmaps;
use super::framework::test_expectations::*;
use crate::rml::parser::extractors::tests::framework::test_core::expect_parse_fail;

#[cfg(test)]    
mod csv_testcases {
    use super::*;

    #[test]
    fn test_rmltc0000() {
        test_triplesmaps(
            "csv-testcases/RMLTC0000-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0001a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0001a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0001b() {
        test_triplesmaps(
            "csv-testcases/RMLTC0001b-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("{Name}"),
                    expect_subject_map_term_type("http://www.w3.org/ns/r2rml#BlankNode"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0002a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://example.com/id"),
                    expect_object_reference_from_pom(0, "ID"),
                    expect_predicate_from_pom(1, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(1, "Name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{ID}/{Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002b() {
        test_triplesmaps(
            "csv-testcases/RMLTC0002b-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("students{ID}"),
                    expect_subject_map_term_type("http://www.w3.org/ns/r2rml#BlankNode"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002c() {
        test_triplesmaps(
            "csv-testcases/RMLTC0002c-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://example.com/id"),
                    expect_object_reference_from_pom(0, "IDs"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{ID}/{Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0002e() {
        test_triplesmaps(
            "csv-testcases/RMLTC0002e-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://example.com/id"),
                    expect_object_reference_from_pom(0, "IDs"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{ID}/{Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0003c() {
        test_triplesmaps(
            "csv-testcases/RMLTC0003c-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0004a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0004a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Student"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{Student}"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Sport"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{Sport}"),
                ]),
            ]
        );
    }

    #[test]
    fn test_rmltc0004b() {
        expect_parse_fail("csv-testcases/RMLTC0004b-CSV.ttl");
    }

    #[test]
    fn test_rmltc0005a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0005a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://example.com/owes"),
                    expect_object_reference_from_pom(0, "amount"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{fname};{lname}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0006a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0006a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(1),
                    expect_predicate_from_pom(0, "http://example.com/description"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0007a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0007a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/Student/{ID}/{FirstName}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0008a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0008a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(4),
                    expect_predicate_from_pom(0, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
                    expect_predicate_from_pom(1, "http://example.com/id"),
                    expect_object_reference_from_pom(1, "ID"),
                    expect_predicate_from_pom(2, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(2, "Name"),
                    expect_predicate_from_pom(3, "http://example.com/Sport"),
                    expect_object_reference_from_pom(3, "Sport"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/Student/{ID}/{Name}"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0009a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0009a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_object_reference_from_pom(0, "Name"),
                    expect_predicate_from_pom(1, "http://example.com/ontology/practises"),
                    expect_has_ref_object_map_in_pom(1, true),
                    expect_ref_object_map_parent_triples_map_from_pom(1, "http://example.com/base/TriplesMap2"),
                    expect_join_condition_child_from_pom(1, "Sport"),
                    expect_join_condition_parent_from_pom(1, "ID"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/resource/student_{ID}"),
                ]),
                ("http://example.com/base/TriplesMap2", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/2000/01/rdf-schema#label"),
                    expect_object_reference_from_pom(0, "Name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/resource/sport_{ID}"),
                ]),
            ]
        );
    }

    #[test]
    fn test_rmltc0010a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0010a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://example.com/name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0012a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0012a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_object_maps_count(2),
                    expect_predicate_from_pom(0, "http://xmlns.com/foaf/0.1/name"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                ])
            ]
        );
    }

    #[test]
    fn test_rmltc0015a() {
        test_triplesmaps(
            "csv-testcases/RMLTC0015a-CSV.ttl",
            vec![
                ("http://example.com/base/TriplesMap1", vec![
                    expect_predicate_from_pom(0, "http://www.w3.org/2000/01/rdf-schema#label"),
                    expect_logical_source_reference_formulation("http://semweb.mmlab.be/ns/ql#CSV"),
                    expect_subject_map_template("http://example.com/{Code}"),
                ]),
            ]
        );
    }

}
