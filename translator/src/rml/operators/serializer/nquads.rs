use std::collections::HashSet;

use log::debug;
use operator::formats::DataFormat;
use vocab::ToString as _;

use super::SerializeTranslator;
use crate::rml::operators::serializer::util::get_triples_strings;
use crate::rml::types::Quad;

#[derive(Debug, Clone)]
pub struct NQuadsSerializer {}

impl SerializeTranslator for NQuadsSerializer {
    fn data_format() -> DataFormat {
        DataFormat::NQuads
    }

    fn generate_template(
        quads: &HashSet<Quad>,
        variable_map: &std::collections::HashMap<String, String>,
    ) -> HashSet<String> {
        let mut quad_strings: HashSet<String> = HashSet::new();
        for quad in quads {
            let mut unterminated_triples =
                get_triples_strings(quad, variable_map).into_iter();

            // TODO: Properly handle overlapping graph maps definition when generating quad
            // patterns  <11-06-24, Min Oo> //
            if quad.gm_opt.is_none() {
                quad_strings
                    .extend(unterminated_triples.map(|trip| trip + " ."));
            } else if let Some(gm) = quad.gm_opt {
                let gm_var = variable_map.get(&gm.tm_info.identifier).unwrap();
                if quad
                    .triple
                    .sm
                    .graph_maps
                    .iter()
                    .map(|gm| &gm.tm_info.identifier)
                    .filter(|id| *id == &gm.tm_info.identifier)
                    .count()
                    == 0
                {
                    unterminated_triples = unterminated_triples
                        .filter(|trip| {
                            !trip.contains(&format!(
                                "<{}>",
                                vocab::rdf::PROPERTY::TYPE.to_string()
                            ))
                        })
                        .collect::<Vec<_>>()
                        .into_iter();
                }

                let quads_with_gm = unterminated_triples
                    .map(|trip| format!("{} {} .", trip, gm_var));
                quad_strings.extend(quads_with_gm);
            }
        }
        debug!("{:#?}", quad_strings);
        quad_strings
    }
}
