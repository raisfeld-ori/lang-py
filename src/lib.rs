use pyo3::prelude::*;
mod entry;

#[pyfunction]
fn parse(text: String) -> PyResult<String> {
    Ok(entry::parse_text(text))
}

// the header file for all rust code.
#[pymodule]
fn rust_header(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(parse))?;
    Ok(())
}
