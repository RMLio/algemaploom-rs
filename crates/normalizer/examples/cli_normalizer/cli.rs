use clap::{arg, ArgMatches, Command};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cli {
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Cli {
        let cmd = Command::new("RML Normalizer")
            .version(VERSION)
            .author("Sitt Min Oo")
            .about(format!("Normalizes an RML document"))
            .propagate_version(true)
            .arg_required_else_help(true)
            .arg(arg!(<INPUT_RML> ... "Input RML document to be normalized"))
            .arg(
                arg!(-d --debug ...  "Turns on debugging and logging to file")
                    .action(clap::ArgAction::SetTrue),
            );

        let matches = cmd.get_matches();
        Self { matches }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
