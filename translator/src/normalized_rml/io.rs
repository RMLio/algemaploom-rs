use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use oxigraph::io::{RdfFormat, RdfSerializer};
use oxigraph::model::GraphNameRef;
use oxigraph::store::Store;

use crate::normalized_rml::error::fromfile::{FromFileError, FromFileErrorKind};

pub fn read_rml_document(path: &Path) -> Result<Store> {
    if let Some(extension) = path.extension() {
        if extension != "ttl" {
            return Err(FromFileError {
                path: path.to_path_buf(),
                kind: FromFileErrorKind::UnsupportedExtension,
            }
            .into());
        }

        let file = File::open(path)
            .map_err(|err| FromFileError::from_io_err(path, err))?;

        let buf_read = BufReader::new(file);
        let store = Store::new()?;
        store
            .load_from_reader(RdfFormat::Turtle, buf_read)
            .map_err(|err| FromFileError::from_io_err(path, err.into()))?;

        Ok(store)
    } else {
        Err(FromFileError {
            path: path.to_path_buf(),
            kind: FromFileErrorKind::UnsupportedExtension,
        }
        .into())
    }
}

pub fn dump_oxigraph_store(store: &Store, path: &Path) -> Result<()> {
    let dump_file = File::create(path)
        .map_err(|err| FromFileError::from_io_err(path, err))?;

    let writer = BufWriter::new(dump_file);
    let serializer: RdfSerializer = RdfFormat::Turtle.into();

    let prefixed_serializer = serializer
        .with_prefix(vocab::rdf::PREFIX, vocab::rdf::IRI)?
        .with_prefix(vocab::r2rml::PREFIX, vocab::r2rml::IRI)?
        .with_prefix(vocab::rml::PREFIX, vocab::rml::IRI)?;

    store
        .dump_graph_to_writer(
            GraphNameRef::DefaultGraph,
            prefixed_serializer,
            writer,
        )
        .map_err(|err| FromFileError::from_io_err(path, err.into()))?;

    Ok(())
}
