use plan::states::Init;
use plan::Plan;

pub mod rml;
pub mod shexml;
//pub mod error; 
//pub mod mapping_test;

#[cfg(test)]
mod test_macro;

pub type LanguageTranslateResult<E> = Result<Plan<Init>, E>;

pub trait LanguageTranslator<T, E> {
    fn translate_to_plan(model: T) -> LanguageTranslateResult<E>;
}

pub trait OperatorTranslator<Output> {
    fn translate(&self) -> Output;
}
