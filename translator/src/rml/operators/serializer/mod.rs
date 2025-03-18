mod nquads;
mod ntriples;
mod util;

use std::collections::{HashMap, HashSet};

use log::debug;
use operator::formats::DataFormat;
use operator::Serializer;

use self::nquads::NQuadsSerializer;
use self::ntriples::NTriplesSerializer;
use crate::rml::types::Quad;

trait SerializeTranslator {
    fn data_format() -> DataFormat;
    fn generate_template(
        quads: &HashSet<Quad>,
        variable_map: &HashMap<String, String>,
    ) -> HashSet<String>;
    fn translate(
        quads: &HashSet<Quad>,
        variable_map: &HashMap<String, String>,
    ) -> Serializer{
        let template_set = Self::generate_template(quads, variable_map);
        let mut template_vec = template_set.into_iter().collect::<Vec<_>>();
        template_vec.sort();

        Serializer{
            template: template_vec.join("\n"),
            //TODO: Check for serializer depdendent options configuration
            options: None,
            format: Self::data_format(),
        }

    }
}

pub fn translate_serializer_op(
    quads: &HashSet<Quad>,
    serialize_format: &DataFormat,
    variable_map: &HashMap<String, String>,
) -> Serializer {
    match serialize_format {
        DataFormat::NTriples => {
            NTriplesSerializer::translate(quads, variable_map)
        }
        DataFormat::NQuads => NQuadsSerializer::translate(quads, variable_map),
        DataFormat::SQL => unimplemented!(),
        DataFormat::JSONLD => unimplemented!(),
        DataFormat::JSON => unimplemented!(),
        DataFormat::XML => unimplemented!(),
        DataFormat::CSV => unimplemented!(),
        DataFormat::TTL => unimplemented!(),
    }
}
