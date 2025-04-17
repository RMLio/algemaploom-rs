use std::path::PathBuf;

use log::{debug, error};

use crate::handler::{FileTranslatorHandler, StringTranslatorHandler};
use crate::rml::{RMLFileHandler, RMLStringHandler};
use crate::shexml::{ShExMLFileHandler, ShExMLStringHandler};
use crate::util::{pretty_print_err, serialize_and_log_msg};

pub fn process_one_file(
    file_path: PathBuf,
    output_prefix: Option<String>,
    json_only_flag: bool,
) {
    let handlers: Vec<Box<dyn FileTranslatorHandler>> =
        vec![Box::new(RMLFileHandler), Box::new(ShExMLFileHandler)];

    if !handlers
        .iter()
        .any(|handler| handler.can_handle(&file_path.to_string_lossy()))
    {
        debug!(
            "Skipped processing file {} since it is not supported",
            file_path.to_string_lossy()
        );
        return; 
    }

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
                pretty_print_err(Box::new(&err));
            });
    } else {
        for mut plan in generated_plans.into_iter().flat_map(|p_res| p_res.ok())
        {
            if let Err(err) = serialize_and_log_msg(
                output_prefix.clone().unwrap(),
                &mut plan,
                file_path.to_string_lossy(),
                json_only_flag,
            ) {
                error!(
                    "Errored while serializing mapping plan for: {}",
                    file_path.to_string_lossy()
                );
                pretty_print_err(Box::new(&err));
            }
        }
    };
}

pub fn process_one_str(mapping: &str) -> String {
    let handlers: Vec<Box<dyn StringTranslatorHandler>> =
        vec![Box::new(RMLStringHandler), Box::new(ShExMLStringHandler)];

    let (generated_plans, generated_errors_res): (Vec<_>, Vec<_>) = handlers
        .iter()
        .map(|handler| handler.translate(mapping))
        .partition(|plan| plan.is_ok());

    if generated_plans.is_empty() {
        if !generated_errors_res.is_empty() {
            error!("Errored while translating from stdin");
        }
        generated_errors_res
            .into_iter()
            .flat_map(|pe| pe.err())
            .enumerate()
            .for_each(|(id, err)| {
                error!("Handler is: {:?} ", handlers[id]);
                pretty_print_err(Box::new(&err));
            });
    } else if let Some(plan) = generated_plans
        .into_iter()
        .flat_map(|p_res| p_res.ok())
        .next()
    {
        return plan.to_string().unwrap();
    };

    panic!("Generated plan not serialized as string")
}
