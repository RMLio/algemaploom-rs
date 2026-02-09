use std::fmt::Display;

pub mod source;
pub mod target;

#[derive(Debug, Clone)]
pub struct RelativePath {
    pub root: PathRootKind,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum PathRootKind {
    CurrentWorkingDirectory,
    MappingDirectory,
    RootString(String),
}

impl Default for PathRootKind {
    fn default() -> Self {
        Self::MappingDirectory
    }
}

impl Display for PathRootKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathRootKind::CurrentWorkingDirectory => {
                write!(
                    f,
                    "{}{}",
                    vocab::rml_io::CLASS::CURRENT_WORKING_DIR.0,
                    vocab::rml_io::CLASS::CURRENT_WORKING_DIR.1
                )
            }
            PathRootKind::MappingDirectory => {
                write!(
                    f,
                    "{}{}",
                    vocab::rml_io::CLASS::MAPPING_DIR.0,
                    vocab::rml_io::CLASS::MAPPING_DIR.1
                )
            }
            PathRootKind::RootString(path) => write!(f, "{}", path),
        }
    }
}
