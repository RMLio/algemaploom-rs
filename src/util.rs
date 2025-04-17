use std::error::Error;

use colored::Colorize;
use log::{error, info};
use plangenerator::error::PlanError;
use plangenerator::states::Init;
use plangenerator::Plan;

pub fn pretty_print_err(err: &dyn Error) {
    error!("Error: {:#}", err);
    if let Some(inner) = err.source() {
        error!("Caused by: ");
        nested_pretty_print_err(0, inner);
    }
}

fn nested_pretty_print_err(mut idx: u32, err: &dyn Error) {
    error!("{}: {:#}", idx, err);
    if let Some(inner) = err.source() {
        idx += 1;
        nested_pretty_print_err(idx, inner);
    }
}

pub fn serialize_and_log_msg<F: AsRef<str>>(
    output_prefix: String,
    mapping_plan: &mut Plan<Init>,
    file: F,
    json_only_flag: bool,
) -> Result<(), PlanError> {
    info!("Translated file: {}", file.as_ref().yellow(),);

    if !json_only_flag {
        let full_path = output_prefix.clone() + ".dot";
        mapping_plan
            .write(full_path.clone().into())
            .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;
        info!("Generated dot file: {}", full_path.yellow());

        let pretty_path = output_prefix.clone() + "_pretty.dot";
        mapping_plan
            .write_pretty(pretty_path.clone().into())
            .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;
        info!(
            "The pretty dot file version for visualization is: {}",
            pretty_path.yellow()
        );
    }

    let json_path = output_prefix + ".json";
    mapping_plan
        .write_json(json_path.clone().into())
        .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;
    info!("Generated json file: {}", json_path.yellow());
    Ok(())
}
