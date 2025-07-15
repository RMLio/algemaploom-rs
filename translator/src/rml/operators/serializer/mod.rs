mod nquads;
mod ntriples;
mod util;

use std::collections::{HashMap, HashSet};

use operator::formats::DataFormat;
use operator::Serializer;

use self::nquads::NQuadsSerializer;
use self::ntriples::NTriplesSerializer;
use crate::rml::types::Quad;
use crate::rml::parser::rml_model::term_map::SubjectMap;

trait SerializeTranslator {
    fn data_format() -> DataFormat;
    fn generate_template(
        quads: &HashSet<Quad>,
        variable_map: &HashMap<String, String>,
        subject_maps: &[&SubjectMap],
    ) -> HashSet<String>;
    fn translate(
        quads: &HashSet<Quad>,
        variable_map: &HashMap<String, String>,
        subject_maps: &[&SubjectMap],
    ) -> Serializer {
        let template_set = Self::generate_template(quads, variable_map, subject_maps);
        let mut template_vec = template_set.into_iter().collect::<Vec<_>>();
        template_vec.sort();

        Serializer {
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
    subject_maps: &[&SubjectMap],
) -> Serializer {
    match serialize_format {
        DataFormat::NTriples => {
            NTriplesSerializer::translate(quads, variable_map, subject_maps)
        }
        DataFormat::NQuads => NQuadsSerializer::translate(quads, variable_map, subject_maps),
        DataFormat::SQL => unimplemented!(),
        DataFormat::JSONLD => unimplemented!(),
        DataFormat::JSON => unimplemented!(),
        DataFormat::XML => unimplemented!(),
        DataFormat::CSV => unimplemented!(),
        DataFormat::TTL => unimplemented!(),
    }
}
