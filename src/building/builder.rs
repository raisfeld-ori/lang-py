use pyo3::prelude::*;
use crate::building::structure::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Builder{
    pub text: String,
    pub start: String,
    pub end: String,
    pub structure: Structure,
}

#[pymethods]
impl Builder {
    pub fn text(&self) -> String {self.text.clone()}

    #[staticmethod]
    pub fn with_std(text: String) -> Builder {
    return Builder {
        text: text,
        start: String::new(),
        end: String::new(),
        structure: Structure::empty(),
    };
}
}