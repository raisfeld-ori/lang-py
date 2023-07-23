use pyo3::prelude::*;

#[pyclass]
pub struct NotVarError (pub String);

#[pymethods]
impl NotVarError {
    pub fn to_string(&self) -> String {return format!("{}: {}", "NotVarError", self.0)}
}