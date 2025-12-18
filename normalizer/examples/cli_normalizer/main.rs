mod cli;
mod logger;

use std::path::{Path, PathBuf};

use anyhow::Result;
use cli::Cli;
use logger::init_logger;

pub fn main() -> Result<()> {
    let cli = Cli::new();
    let debug_enabled = cli.matches.get_flag("debug");
    init_logger(debug_enabled)?;

    let input_rml_path = cli
        .matches
        .get_one::<PathBuf>("INPUT_RML")
        .expect("Input RML document is required");

    normalizer::normalize_rml_path(input_rml_path)?;
    Ok(())
}
