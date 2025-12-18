use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use error::NormalizerResult;
use oxigraph::io::RdfFormat;
use oxigraph::sparql::SparqlEvaluator;
use oxigraph::store::Store;
use queries::QUERY_MSG_PAIRS;

mod error;
mod queries;

pub fn normalize_rml_str(rml_str: &str) -> NormalizerResult<String> {
    let store = Store::new().unwrap();
    store.load_from_reader(RdfFormat::Turtle, rml_str.as_bytes())?;
    for (update, msg) in QUERY_MSG_PAIRS {
        log::info!("{}", msg);
        let update = SparqlEvaluator::new().parse_update(update)?;
        update.on_store(&store).execute()?;
    }

    let mut buf_writer = BufWriter::new(vec![]);
    buf_writer = store.dump_to_writer(RdfFormat::Turtle, buf_writer)?;

    Ok(String::from_utf8(buf_writer.into_inner().unwrap())?)
}

pub fn normalize_rml_path(rml_path: &Path) -> NormalizerResult<()> {
    log::info!(
        "Normalizing the given RML document at {}",
        rml_path.to_string_lossy()
    );
    let file = File::open(rml_path)?;
    let mut reader = BufReader::new(file);
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    let normalized_rml = normalize_rml_str(String::from_utf8(buf)?.as_str())?;

    let output_file_name = format!(
        "normalized_{}",
        rml_path
            .file_name()
            .expect("Expect a valid file name for the given path")
            .to_string_lossy()
    );
    let output_path = rml_path.with_file_name(output_file_name);
    let output_file = File::create(output_path)?;
    let mut buf_writer = BufWriter::new(output_file);
    buf_writer.write_all(normalized_rml.as_bytes())?;
    Ok(())
}
