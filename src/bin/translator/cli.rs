use clap::{arg, Command};

pub const TRANSLATOR_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cli {
    pub cmd: Command,
}

impl Cli {
    pub fn new() -> Cli {
        let cmd = Command::new("AlgeMapLoom-rs translator ")
            .version(TRANSLATOR_VERSION)
            .author("Sitt Min Oo")
            .about(format!("Translates mapping documents to execution plans consisting of algebraic mapping operators.\n\
                Current version {} supports RML and ShExML mapping languages.", TRANSLATOR_VERSION))
            .subcommand_required(true)
            .propagate_version(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("file")
                         .about("translate a single mapping document")
                         .arg(arg!(<RML_DOCUMENT> "the mapping document to be translated"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("folder")
                         .about("translate all mapping documents under the given folder")
                         .arg(arg!(<FOLDER> "the folder containing several mapping documents"))
                         .arg_required_else_help(true))
            .arg(arg!(-o --outputFolderSuffix <OUTPUT_FOLDER_SUFFIX> "The output folder suffix"));

        Self { cmd }
    }
}
