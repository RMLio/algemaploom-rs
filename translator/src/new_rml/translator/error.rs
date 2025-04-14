use std::fmt::Display;

#[derive(Debug)]
pub enum TranslationError {
    SourceError(String),
    ExtendError(String), 
    Infallible,
}


impl Display for TranslationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl std::error::Error for TranslationError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}
