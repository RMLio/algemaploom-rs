#!/usr/bin/env python3

"""
File: normalizer.py
Author: Anon
Email: Anon
Github: Anon
Description: Normalize the input RML documents
"""

import argparse
import logging
import os
import pathlib
import sys
from typing import List, Literal

import rdflib
from rdflib.graph import Graph

logger = logging.getLogger(__name__)


def cmdline_args():
    # Make parser object
    p = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )

    p.add_argument("-f", "--file", type=str, help="input RML document file")
    p.add_argument("--folder", type=str, help="the folder containing RML documents")
    p.add_argument("-d", "--debug", action="store_true", help="toggle debug mode")

    return p.parse_args()


def end_log(g: Graph):
    logger.debug("\n" + g.serialize())
    logger.debug("=" * 20)
    pass


def class_shortcut_expand(g: Graph):
    logger.debug("Expand subject map's class shortcut")
    g.update(
        """
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

             """
    )
    end_log(g)
    pass


def shortcut_expand_to_constant_tm(g: Graph):
    logger.debug("Expand shortcuts to constant term maps")
    g.update(
        """
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
        """
    )
    end_log(g)
    pass


def multiple_pm_om_to_pom_singleton_pm_om(g: Graph):
    logger.debug(
        "Make predicate object maps have only one predicate map and one object map"
    )
    g.update(
        """
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
             """
    )

    end_log(g)
    pass


def replace_self_reference_obj_map(g: Graph):
    logger.debug(
        "Replcae self referencing object map with a simple predicate object map"
    )
    g.update(
        """
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
             """
    )
    end_log(g)
    pass


def tm_multiple_pom_to_single_pom(g: Graph):
    logger.debug("Ensure triples maps only have a single predicate object map")
    g.update(
        """
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
             """
    )
    end_log(g)
    pass


def push_pom_gm_to_sm(g: Graph):
    logger.debug("Pushing graph maps in predicate object maps to subject maps")
    g.update(
        """
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
             """
    )
    end_log(g)
    pass


def multiple_sm_gm_to_single_sm_gm(g: Graph):
    logger.debug("Final step of flattening subject map's graph maps ")
    g.update(
        """
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
             """
    )
    end_log(g)
    pass


def handle_file(file: str) -> str:

    if file.endswith(".ttl"):
        logger.info("Processing file " + file)
        base_path = file[0:-4]
        output_file = base_path + ".normalized.ttl"
        base_iri = None

        with open(file, "r") as f:
            for line in f.readlines():
                if "@base" in line:
                    base_iri = line
                pass
            pass
        try:
            os.remove(output_file)
        except Exception as e:
            pass

        g = rdflib.Graph().parse(file)
        class_shortcut_expand(g)
        shortcut_expand_to_constant_tm(g)
        multiple_pm_om_to_pom_singleton_pm_om(g)
        replace_self_reference_obj_map(g)
        tm_multiple_pom_to_single_pom(g)
        push_pom_gm_to_sm(g)
        multiple_sm_gm_to_single_sm_gm(g)

        g.serialize(output_file)
        logger.info("Done normalizing file " + file)

        content = ""
        if base_iri is not None:
            content = base_iri + "\n"

        content = (
            content + "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n"
        )
        with open(output_file, "r") as f:
            content = content + f.read()
            pass

        with open(output_file, "w") as f:
            f.write(content)
            pass

        logger.info("Normalized file written to" + output_file)
        return output_file

    return ""


def collect_files(folder: str) -> List[str]:
    acc = []
    for root, dirs, files in os.walk(folder):
        for file in files:
            acc.append(os.path.join(root, file))
            pass
        for dir in dirs:
            acc.extend(collect_files(os.path.join(root, dir)))
            pass
        pass
    return acc


def handle_folder(folder: str):
    for file in collect_files(folder):
        handle_file(file)
    pass


def logging_setup(log_level: int):
    format = "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
    formatter = logging.Formatter(format)

    std_handler = logging.StreamHandler()
    std_handler.setLevel(log_level)
    std_handler.setFormatter(formatter)

    file_handler = logging.FileHandler(filename="log_normalizer.log", mode="w+")
    file_handler.setLevel(log_level)
    file_handler.setFormatter(formatter)

    logger.setLevel(log_level)
    logger.addHandler(std_handler)
    logger.addHandler(file_handler)
    pass


def main():
    args = cmdline_args()
    log_level = logging.INFO
    if args.debug:
        log_level = logging.DEBUG

    logging_setup(log_level)

    if args.file is not None:
        handle_file(args.file)
        pass
    elif args.folder is not None:
        handle_folder(args.folder)
        pass
    pass


if __name__ == "__main__":
    main()
