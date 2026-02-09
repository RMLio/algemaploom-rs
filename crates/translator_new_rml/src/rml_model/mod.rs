use v2::core::TriplesMap;

//pub mod v1;
pub mod v2;

#[derive(Debug, Clone)]
pub struct Document {
    pub default_base_iri: Option<String>,
    pub triples_maps:     Vec<TriplesMap>,
}

impl Document {
    pub fn new(
        default_base_iri: Option<String>,
        triples_maps: Vec<TriplesMap>,
    ) -> Self {
        let base_iri = default_base_iri.clone().unwrap_or_default();
        let triples_maps = triples_maps
            .into_iter()
            .map(|mut tm| {
                if tm.base_iri.is_empty() {
                    tm.base_iri = base_iri.clone();
                }
                tm
            })
            .collect();

        Self {
            default_base_iri,
            triples_maps,
        }
    }
}
