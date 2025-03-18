#[macro_export]
macro_rules! test_case {($fname:expr) => (
  concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname) // assumes Linux ('/')!
)}

#[macro_export]
macro_rules! load_graph {
    ($fname:expr) => {{
        let path = test_case!($fname);
        let pathbuf = PathBuf::from(path);
        let bread = BufReader::new(File::open(pathbuf)?);
        load_graph_bread(bread)
    }};
}

#[macro_export]
macro_rules! import_test_mods {
    () => {
        use std::fs::File;
        use std::io::BufReader;
        use std::path::PathBuf;

        use $crate::rml::parser::extractors::io::load_graph_bread;
        use $crate::rml::parser::extractors::ExtractorResult;
        use $crate::{load_graph, test_case};
    };
}
