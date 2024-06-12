use std::path::PathBuf;
use log::error;
use crate::handler::{FileTranslatorHandler, StringTranslatorHandler};
use crate::serialize_and_log_msg;

pub fn process_one_file(
    handlers: &[Box<dyn FileTranslatorHandler>],
    file_path: PathBuf,
    output_prefix: Option<String>,
) {
    let (generated_plans, generated_errors_res): (Vec<_>, Vec<_>) = handlers
        .iter()
        .map(|handler| handler.handle_file(&file_path.to_string_lossy()))
        .partition(|plan| plan.is_ok());
    if generated_plans.is_empty() {
        if !generated_errors_res.is_empty() {
            error!(
                "Errored while translating: {}",
                file_path.to_string_lossy()
            );
        }
        generated_errors_res
            .into_iter()
            .flat_map(|pe| pe.err())
            .enumerate()
            .for_each(|(id, err)| {
                error!("Handler is: {:?} ", handlers[id]);
                error!("{}", err);
            });
    } else {
        for mut plan in generated_plans.into_iter().flat_map(|p_res| p_res.ok())
        {
            if let Err(err) = serialize_and_log_msg(
                output_prefix.clone().unwrap(),
                &mut plan,
                file_path.to_string_lossy(),
            ) {
                error!(
                    "Errored while serializing mapping plan for: {}",
                    file_path.to_string_lossy()
                );
                error!("{}", err);
            }
        }
    };
}

pub fn process_one_str(
    handlers: &[Box<dyn StringTranslatorHandler>],
    mapping: &str) -> String {

    let (generated_plans, generated_errors_res): (Vec<_>, Vec<_>) = handlers
        .iter()
        .map(|handler| handler.translate(mapping))
        .partition(|plan| plan.is_ok());

    if generated_plans.is_empty() {
        if !generated_errors_res.is_empty() {
            error!(
                "Errored while translating from stdin"
            );
        }
        generated_errors_res
            .into_iter()
            .flat_map(|pe| pe.err())
            .enumerate()
            .for_each(|(id, err)| {
                error!("Handler is: {:?} ", handlers[id]);
                error!("{}", err);
            });
    } else {
        for plan in generated_plans.into_iter().flat_map(|p_res| p_res.ok())
        {
            return plan.to_string().unwrap()
        }
    };

    panic!("Generated plan not serialized as string")
}
