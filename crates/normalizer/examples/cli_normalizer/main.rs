mod cli;

use std::path::Path;

use anyhow::Result;
use cli::Cli;
use common::logger::init_logger;

pub fn main() -> Result<()> {
    let cli = Cli::new();
    let debug_enabled = cli.matches.get_flag("debug");
    init_logger(debug_enabled)?;

    let input_rml_path = cli
        .matches
        .get_one::<String>("INPUT_RML")
        .expect("Input RML document is required");

    normalizer::normalize_rml_path(Path::new(&input_rml_path))?;
    Ok(())
}
