pub const PREFIX: &str = "ldes";
pub const IRI: &str = "https://w3id.org/ldes#";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const EVENTSTREAM: PAIR = (IRI, "EventStream");
}

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const TIMESTAMPPATH: PAIR = (IRI, "timestampPath");
    pub const VERSIONOFPATH: PAIR = (IRI, "versionOfPath");
} 