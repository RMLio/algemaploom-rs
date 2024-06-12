use colored::Colorize;
use log::info;
use plangenerator::error::PlanError;
use plangenerator::plan::{Init, Plan};

pub fn serialize_and_log_msg<F: AsRef<str>>(
    output_prefix: String,
    mapping_plan: &mut Plan<Init>,
    file: F,
) -> Result<(), PlanError> {
    let full_path = output_prefix.clone() + ".dot";
    mapping_plan
        .write(full_path.clone().into())
        .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;
    let pretty_path = output_prefix.clone() + "_pretty.dot";
    mapping_plan
        .write_pretty(pretty_path.clone().into())
        .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;
    let json_path = output_prefix + ".json";
    mapping_plan
        .write_json(json_path.clone().into())
        .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;
    info!(
        "Translated file: {}",
        file.as_ref().yellow(),
    );
    info!("Generated dot file: {}", full_path.yellow());
    info!(
        "The pretty dot file version for visualization is: {}",
        pretty_path.yellow()
    );
    info!("Generated json file: {}", json_path.yellow());
    Ok(())
}
