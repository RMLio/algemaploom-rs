use pyo3::prelude::*;
use crate::api::process_one_str;

#[pyfunction]
fn translate(mapping: String) -> PyResult<String> {
    let translated = process_one_str(mapping.as_str())
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Translation failed"))?;

    Ok(translated)
}

#[pymodule]
fn translator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(translate, m)?)?;
    Ok(())
}
