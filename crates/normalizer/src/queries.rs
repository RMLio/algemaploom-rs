pub const QUERY_MSG_PAIRS: &[(&str, &str)] = 
    &[(
        CLASS_SHORTCUT_EXPAND,
        "Expand subject map's class shortcut to separate predicate-object map",
    ),
    (   SHORTCUT_EXPAND_TO_CONSTANT_TERM, 
        "Expand term map's constant shortcuts"
    ),
    (
        MULTIPLE_PM_OM_TO_SINGLE_PM_OM, 
        "Rewrite POMs such that each of them only contain a single predicate and a single object map"
    ),
    (  REPLACE_SELF_REFERENCING_OBJ_MAP, 
       "Replace (obvious) self referencing joins such that object map becomes the subject map of the parent triples map"
        ), 

    (
        MULTIPLE_POM_TO_SINGLE_POM, 
        "Flatten multiple POMs such that each Triples Map only contain one POM"
        ),
    (
        PUSH_POM_GM_TO_SM, 
        "Push graph map inside POM to the subject map"
        ), 
    (
        MULTIPLE_SM_GM_TO_SINGLE_SM_GM, 
        "Flatten multiple graph maps in subject map such that each subject map has only one graph map"

        )
    ];

pub const CLASS_SHORTCUT_EXPAND: &str = "

        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 

        DELETE { ?sm rr:class ?sm_class . }
        INSERT {
             ?tm rr:predicateObjectMap [
                 rr:predicateMap [
                     rr:constant rdf:type; 
                     rr:termType rr:IRI 
                 ]; 
                 rr:objectMap  [
                     rr:constant ?sm_class; 
                     rr:termType rr:IRI
                 ]
             ]
        }
        WHERE {
            ?tm rr:subjectMap ?sm . 
            ?sm rr:class ?sm_class . 
        }
";

pub const SHORTCUT_EXPAND_TO_CONSTANT_TERM: &str = "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        DELETE { 
            ?tm rr:subject ?sm_constant.
            ?pompm rr:predicate ?pm_constant.
            ?pomom rr:object ?om_constant. 
            ?termMap rr:graph ?gm_constant.
        }
        INSERT {
             ?tm rr:subjectMap [ 
                rr:constant ?sm_constant
             ]. 

             ?pompm rr:predicateMap [
                rr:constant ?pm_constant
             ]. 

             ?pomom rr:objectMap [
                rr:constant ?om_constant
             ].

             ?termMap rr:graphMap [
                rr:constant ?gm_constant
             ].
        }
        WHERE {
                    { ?tm rr:subject ?sm_constant . } 
            UNION   { ?pompm rr:predicate ?pm_constant . } 
            UNION   { ?pomom rr:object ?om_constant . } 
            UNION   { ?termMap rr:graph ?gm_constant . } 
        }
";

pub const MULTIPLE_PM_OM_TO_SINGLE_PM_OM: &str = "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom. 
            ?pom rr:predicateMap ?pm; 
                 rr:objectMap ?om; 
                 rr:graphMap ?gm. 
        }
        INSERT {
            ?tm rr:predicateObjectMap [
                rr:predicateMap ?pm; 
                rr:objectMap ?om; 
                rr:graphMap ?gm
            ]
        }
        WHERE {
            ?tm rr:predicateObjectMap ?pom. 
            ?pom rr:predicateMap ?pm; 
                 rr:objectMap ?om. 
            
            OPTIONAL {
                ?pom rr:graphMap ?gm. 
            }
        }
";

pub const REPLACE_SELF_REFERENCING_OBJ_MAP: &str = "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?om rr:parentTriplesMap ?ptm 
        }
        INSERT {
            ?om rml:reference ?ref ; 
                rr:template ?template ;
                rr:constant ?const; 
                rr:termType rr:IRI 
        }
        WHERE {
            ?om rr:parentTriplesMap ?ptm . 
            ?ptm rr:subjectMap ?sm . 
            OPTIONAL { ?sm rr:reference ?ref. }
            OPTIONAL { ?sm rr:template ?template. }
            OPTIONAL { ?sm rr:constant ?const. }
            FILTER NOT EXISTS {
                ?om rr:joinCondition ?jc
            }
        }
";

pub const MULTIPLE_POM_TO_SINGLE_POM: &str = "
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> 
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom
        }
        INSERT {
            [] a rr:TriplesMap; 
                rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom 

        }
        WHERE {
            ?tm rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom 
        }
";

pub const PUSH_POM_GM_TO_SM: &str = "

        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom . 
            ?pom rr:graphMap ?pom_gm .
        }
        INSERT {
            [] a rr:TriplesMap; 
               rml:logicalSource ?ls; 
               rr:subjectMap [
                   rr:reference ?ref; 
                   rr:template ?template; 
                   rr:constant ?const; 
                   rr:termType ?ttype; 
                   rr:graphMap ?pom_gm;
                   rr:graphMap ?sm_gm 
               ]; 
               rr:predicateObjectMap ?pom. 
        }
        WHERE {
            ?tm rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom .

            ?pom rr:graphMap ?pom_gm . 
            OPTIONAL {?sm rr:graphMap ?sm_gm .}

            OPTIONAL { ?sm rr:reference ?ref. }
            OPTIONAL { ?sm rr:template ?template. }
            OPTIONAL { ?sm rr:constant ?const. }
            OPTIONAL { ?sm rr:termType ?ttype. }
        }
";

pub const MULTIPLE_SM_GM_TO_SINGLE_SM_GM: &str = "
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX rr: <http://www.w3.org/ns/r2rml#> 
        PREFIX rml: <http://semweb.mmlab.be/ns/rml#>
        
        DELETE { 
            ?tm rr:predicateObjectMap ?pom .
            ?sm rr:graphMap ?sm_gm .
        }
        INSERT {
            [] a rr:TriplesMap; 
               rml:logicalSource ?ls; 
               rr:subjectMap [
                   rr:reference ?ref; 
                   rr:template ?template; 
                   rr:constant ?const; 
                   rr:termType ?ttype; 
                   rr:graphMap ?sm_gm
               ]; 
               rr:predicateObjectMap ?pom. 
        }
        WHERE {
            ?tm rml:logicalSource ?ls; 
                rr:subjectMap ?sm; 
                rr:predicateObjectMap ?pom .
            ?sm rr:graphMap ?sm_gm . 
            ?sm rr:graphMap ?sm_gm2 .
            FILTER( ?sm_gm != ?sm_gm2 )

            OPTIONAL { ?sm rr:reference ?ref. }
            OPTIONAL { ?sm rr:template ?template. }
            OPTIONAL { ?sm rr:constant ?const. }
            OPTIONAL { ?sm rr:termType ?ttype. }
        }
";
