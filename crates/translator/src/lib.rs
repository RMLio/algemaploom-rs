use error::TranslationError;
use plan::states::Init;
use plan::Plan;

pub mod rml;
pub mod new_rml;
pub mod shexml;
pub mod normalized_rml; 
pub mod error; 
pub mod mapping_test;

#[cfg(test)]
mod test_macro;

pub type LanguageTranslateResult = Result<Plan<Init>, TranslationError>;

pub trait LanguageTranslator<T> {
    fn translate_to_plan(model: T) -> LanguageTranslateResult;
}

pub trait OperatorTranslator<Output> {
    fn translate(&self) -> Output;
}
