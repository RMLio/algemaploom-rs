use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExpressionMapError {
    #[error("invalid template: `{0}` : {1}")]
    InvalidTemplate(String, String),
}