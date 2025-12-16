use std::io::Write;
use std::result::Result;
use std::{fs::File, io::BufWriter, path::PathBuf};

use petgraph::dot::Dot;
use plan::data_type::DiGraphOperators;

pub fn to_pretty_dot_file(graph: &mut DiGraphOperators, path: PathBuf) -> Result<(), std::io::Error> {
    write_fmt(graph, path, &|dot| format!("{}", dot))
}
pub fn to_dot_file(graph: &mut DiGraphOperators, path: PathBuf) -> Result<(), std::io::Error> {
    write_fmt(graph, path, &|dot| format!("{:?}", dot))
}

pub fn to_json_file(graph: &mut DiGraphOperators, path: PathBuf) -> Result<(), std::io::Error> {
    write_string_to_file(path, serde_json::to_string(graph)?)
}

fn write_fmt(
    graph: &mut DiGraphOperators,
    path: PathBuf,
    fmt: &dyn Fn(Dot<&DiGraphOperators>) -> String,
) -> Result<(), std::io::Error> {
    let dot_string = fmt(Dot::with_config(graph, &[]));
    write_string_to_file(path, dot_string)?;
    Ok(())
}

fn write_string_to_file(path: PathBuf, content: String) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", content)?;
    Ok(())
}
