pub const PREFIX: &str = "rml";

pub const IRI: &str = "http://w3id.org/rml/";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const NAMESPACE: PAIR = (IRI, "namespace");
    pub const NAMESPACE_PREFIX: PAIR = (IRI, "namespacePrefix");
    pub const NAMESPACE_URL: PAIR = (IRI, "namespaceURL");

    pub const NULL: PAIR = (IRI, "null");
    pub const SOURCE: PAIR = (IRI, "source");
    pub const ENCODING: PAIR = (IRI, "encoding");
    pub const COMPRESSION: PAIR = (IRI, "compression");
    pub const TARGET: PAIR = (IRI, "target");
    pub const SERIALIZATION: PAIR = (IRI, "serialization");
    pub const ROOT: PAIR = (IRI, "root");
    pub const PATH: PAIR = (IRI, "path");
    pub const MODE: PAIR = (IRI, "mode");
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const CURRENT_WORKING_DIR: PAIR = (IRI, "CurrentWorkingDirectory");
    pub const MAPPING_DIR: PAIR = (IRI, "MappingDirectory");

    pub const UTF_8: PAIR = (IRI, "UTF-8");
    pub const UTF_16: PAIR = (IRI, "UTF-16");

    // Compression formats
    pub const NONE: PAIR = (IRI, "none");
    pub const GZIP: PAIR = (IRI, "gzip");
    pub const ZIP: PAIR = (IRI, "zip");
    pub const TARGZ: PAIR = (IRI, "targz");

    pub const LOGICAL_SOURCE: PAIR = (IRI, "LogicalSource");
    pub const SOURCE: PAIR = (IRI, "Source");

    pub const FILE_PATH: PAIR = (IRI, "FilePath");
    pub const RELATIVE_PATH: PAIR = (IRI, "RelativePath");
    pub const RELATIVE_PATH_SOURCE: PAIR = (IRI, "RelativePathSource");
}
