use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use error::{NormalizerError, NormalizerResult};
use oxigraph::io::{RdfFormat, RdfParseError, RdfParser, RdfSerializer};
use oxigraph::model::Triple;
use oxigraph::sparql::SparqlEvaluator;
use oxigraph::store::Store;
use queries::QUERY_MSG_PAIRS;

mod error;
mod queries;

pub fn normalize_rml_str(rml_str: &str) -> NormalizerResult<String> {
    let store = Store::new()?;
    let mut loader = store.bulk_loader();
    let parser = RdfParser::from_format(RdfFormat::Turtle);
    let mut reader = parser.for_reader(rml_str.as_bytes());

    loader.load_ok_quads::<RdfParseError, NormalizerError>(&mut reader)?;
    loader.commit()?;

    let prefixes = reader.prefixes().collect::<Vec<_>>();
    log::debug!("Found prefixes of the RML document: {:?}", prefixes);

    for (update, msg) in QUERY_MSG_PAIRS {
        log::info!("{}", msg);
        let update = SparqlEvaluator::new().parse_update(update)?;
        update.on_store(&store).execute()?;
    }

    log::info!("Serializing normalized RDF dataset to Turtle");
    let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle);
    serializer = prefixes.iter().fold(serializer, |acc, pref| {
        acc.with_prefix(pref.0, pref.1).unwrap()
    });

    let mut buf_writer = BufWriter::new(vec![]);
    let mut write_serializer = serializer.for_writer(buf_writer);
    for quad in store.iter().flatten() {
        write_serializer.serialize_triple(&Into::<Triple>::into(quad))?;
    }

    buf_writer = write_serializer.finish()?;

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
