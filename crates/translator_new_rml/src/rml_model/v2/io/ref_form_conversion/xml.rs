use operator::formats::xml::Namespaces;

use super::ComplexRefFormulationParser;
use crate::extractors::store::{get_object, get_objects};
use crate::extractors::{stringify_term, FromVocab};

pub struct XPathRefFormParser;

impl ComplexRefFormulationParser for XPathRefFormParser {
    fn parse_complex_ref_form(
        subj_ref: &sophia_term::RcTerm,
        meta_data_graph: &std::rc::Rc<sophia_inmem::graph::FastGraph>,
    ) -> Result<
        operator::formats::ReferenceFormulation,
        crate::extractors::error::ParseError,
    > {
        let namespace_bnodes = get_objects(
            &meta_data_graph,
            subj_ref,
            vocab::rml_io::PROPERTY::NAMESPACE.to_rcterm(),
        );

        let namespace_prefix_iri =
            vocab::rml_io::PROPERTY::NAMESPACE_PREFIX.to_rcterm();
        let namespace_url_iri =
            vocab::rml_io::PROPERTY::NAMESPACE_URL.to_rcterm();
        let namespaces = namespace_bnodes
            .into_iter()
            .map(|bnode| {
                get_object(&meta_data_graph, &bnode, &namespace_prefix_iri)
                    .and_then(|prefix| {
                        get_object(&meta_data_graph, &bnode, &namespace_url_iri)
                            .and_then(|url| Ok((prefix, url)))
                    })
            })
            .collect::<Result<Vec<(_, _)>, _>>()?
            .into_iter()
            .map(|(prefix, url)| {
                Namespaces {
                    prefix: stringify_term(prefix).unwrap(),
                    url:    stringify_term(url).unwrap(),
                }
            })
            .collect::<Vec<_>>();

        Ok(operator::formats::ReferenceFormulation::XMLPath(
            operator::formats::xml::XPathConfig { namespaces },
        ))
    }
}

