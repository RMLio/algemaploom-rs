use plan::states::Init;
use plan::Plan;


#[cfg(test)]
mod test_macro;

pub type LanguageTranslateResult<E> = Result<Plan<Init>, E>;

pub trait LanguageTranslator<T, E> {
    fn translate_to_plan(model: T) -> LanguageTranslateResult<E>;
}

pub trait OperatorTranslator<Output> {
    fn translate(&self) -> Output;
}
