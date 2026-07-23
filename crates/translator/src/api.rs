use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::handler::TranslatorHandler;
use crate::rml::RMLHandler;
use crate::shexml::ShExMLHandler;
use crate::util::{pretty_print_err, serialize_and_log_msg};
use log::error;
use plan::states::Init;
use plan::Plan;

pub fn process_one_file(
    file_path: PathBuf,
    output_prefix: Option<String>,
    json_only_flag: bool,
    pretty: bool,
) {
    let mapping_str_res = fs::read_to_string(&file_path);
    match mapping_str_res {
        Ok(mapping) => {
            if let Some(plan) = process(&mapping) {
                if let Err(err) = serialize_and_log_msg(
                    output_prefix.clone().unwrap(),
                    &plan,
                    file_path.to_string_lossy(),
                    json_only_flag,
                    pretty,
                ) {
                    error!(
                        "Errored while serializing mapping plan for: {}",
                        file_path.to_string_lossy()
                    );
                    pretty_print_err(&err);
                }
            } 
        },
        Err(err) => {
            error!("Errored while reading file: {}", file_path.to_string_lossy());
            pretty_print_err(&err);
        },
    }
}

pub fn process_one_str(mapping: &str) -> Option<String> {
    process(mapping)
        .map_or(None, |plan| Some(plan.to_json_string().unwrap()))
}

fn process(mapping: &str) -> Option<Plan<Init>> {
    let handlers: Vec<Box<dyn TranslatorHandler>> =
        vec![Box::new(RMLHandler), Box::new(ShExMLHandler)];
    let mut error_messages: Vec<String> = Vec::new();

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
                let mut chain: Vec<String> = vec![format!("Error: {:#}", err)];
                let mut idx: u32 = 0;
                let mut current = err.source();
                while let Some(inner) = current {
                    chain.push(format!("{}: {:#}", idx, inner));
                    idx += 1;
                    current = inner.source();
                }
                error_messages.push(chain.join(" | "));
                pretty_print_err(&err);
            });
    } else if let Some(plan) = generated_plans
        .into_iter()
        .flat_map(|p_res| p_res.ok())
        .next()
    {
        return Some(plan);
    };

    let rust_logs = if error_messages.is_empty() {
        "No error messages".to_string()
    } else {
        error_messages.join(" || ")
    };
    error!("Error messages: {}", rust_logs);
    None
}
