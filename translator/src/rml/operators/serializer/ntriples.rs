use std::collections::{HashMap, HashSet};

use operator::formats::DataFormat;

use super::util::get_triples_strings;
use super::SerializeTranslator;
use crate::rml::types::Quad;

#[derive(Debug, Clone)]
pub struct NTriplesSerializer {}

impl SerializeTranslator for NTriplesSerializer {
    fn generate_template(
        quads: &HashSet<Quad>,
        variable_map: &HashMap<String, String>,
    ) -> HashSet<String> {
        let mut triples_strings: HashSet<String> = HashSet::new();
        for quad in quads {
            let terminated_triples =
                get_triples_strings(quad, variable_map)
                    .into_iter()
                    .map(|str| format!("{} .", str));

            triples_strings.extend(terminated_triples);
        }
        triples_strings
    }

    fn data_format() -> DataFormat {
        DataFormat::NTriples
    }
}
