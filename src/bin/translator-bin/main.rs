
mod cli;

use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use log::debug;
use ltranslator::api::{process_one_file, process_one_str};
use ltranslator::logger::init_logger;
use plan::error::PlanError;
use walkdir::WalkDir;

pub fn main() -> Result<(), PlanError> {
    let cli = cli::Cli::new();

    let matches = cli.cmd.get_matches();
    let debug_flag_count = *matches.get_one::<u8>("debug").unwrap();
    let json_only = (*matches.get_one::<u8>("json").unwrap()) >= 1;
    init_logger(debug_flag_count >= 1)
        .map_err(|err| PlanError::GenericError(err.to_string()))?;

    if let Some(file_matches) = matches.subcommand_matches("file") {
        let file_path_string: &String =
            file_matches.get_one("DOCUMENT").unwrap();

        debug!("Attempting to translate: {:?}", file_path_string);
        let file_path: PathBuf = file_path_string.into();
        let mut output_prefix = Some("output".to_string());
        if let Some(derived_prefix) = file_path.file_stem() {
            let derived_string = derived_prefix.to_string_lossy();
            let _ = output_prefix.insert(derived_string.to_string());
        }
        process_one_file(file_path, output_prefix, json_only);
    } else if let Some(folder_matches) = matches.subcommand_matches("folder") {
        let folder_path_string: &String =
            folder_matches.get_one("FOLDER").unwrap();
        let folder_path: PathBuf = folder_path_string.into();
        let files = WalkDir::new(folder_path)
            .max_depth(4)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|dentry| dentry.file_type().is_file());

        for file in files {
            debug!(
                "Attempting to translate: {}",
                file.path().to_string_lossy()
            );
            let input_path = file.path();

            let output_dir = input_path
                .parent()
                .map_or("".to_string(), |p| p.to_string_lossy().to_string());
            let output_prefix = output_dir
                + "/"
                + &input_path.file_stem().unwrap().to_string_lossy();

            process_one_file(input_path.to_path_buf(), Some(output_prefix), json_only);
        }
    } else if let Some(_stdin_matches) = matches.subcommand_matches("stdin") {
        let mut mapping = String::new();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            mapping = String::from(mapping + line.unwrap().clone().as_str());
        }

        debug!("Attempting to translate from stdin");
        let out = process_one_str(mapping.as_str());
        println!("{}", out);
    }

    Ok(())
}
