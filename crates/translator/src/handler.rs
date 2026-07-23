use std::fmt::Debug;

use crate::error::TranslationError;
use plan::states::Init;
use plan::Plan;

pub trait TranslatorHandler: Debug {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError>;
}
