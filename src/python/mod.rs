use pyo3::prelude::*;
use crate::api::process_one_str;

#[pyfunction]
fn translate(mapping: String) -> PyResult<String> {
    let translated = process_one_str(mapping.as_str());

    Ok(translated)
}

#[pymodule]
fn ltranslator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(translate, m)?)?;
    Ok(())
}
