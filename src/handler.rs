use std::fmt::Debug;
use std::path::PathBuf;

use plangenerator::error::PlanError;
use plangenerator::states::Init;
use plangenerator::Plan;
use translator::error::{TranslationError, TranslationErrorKind};

pub trait FileTranslatorHandler: Debug {
    fn supported_extension(&self) -> String;
    fn can_handle(&self, file_path: &dyn AsRef<str>) -> bool {
        let pbuf: PathBuf = file_path.as_ref().into();

        let extension_opt = pbuf.extension();

        if let Some(extension) = extension_opt {
            extension.to_string_lossy() == self.supported_extension()
        } else {
            false
        }
    }
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, TranslationError>;

    fn handle_file(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, TranslationError> {
        if !self.can_handle(file_path) {
            return Err(TranslationError {
                kind: TranslationErrorKind::FileMsgError {
                    file: file_path.as_ref().into(),
                    msg:  format!("Unsupported file for {:?}", self),
                },
            });
        }
        self.translate(file_path)
    }
}

pub trait StringTranslatorHandler: Debug {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError>;
}

pub trait TranslatorHandler:
    FileTranslatorHandler + StringTranslatorHandler
{
}
impl<T> TranslatorHandler for T where
    T: FileTranslatorHandler + StringTranslatorHandler
{
}
