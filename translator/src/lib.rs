use plangenerator::error::PlanError;
use plangenerator::states::Init;
use plangenerator::Plan;

pub mod rmlalgebra;
pub mod shexml;

#[cfg(test)]
mod test_macro;

pub type LanguageTranslateResult = Result<Plan<Init>, PlanError>;

pub trait LanguageTranslator<T> {
    fn translate_to_plan(model: T) -> LanguageTranslateResult;
}

pub trait OperatorTranslator<Output> {
    fn translate(&self) -> Output;
}
