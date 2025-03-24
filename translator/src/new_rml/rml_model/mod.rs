use v2::core::TriplesMap;

//pub mod v1;
pub mod v2;

#[derive(Debug, Clone)]
pub struct Document {
    pub default_base_iri: Option<String>,
    pub triples_maps:     Vec<TriplesMap>,
}
