use pyo3::prelude::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Builder{
    pub text: String,
}

pub fn create_builder(text: String) -> Builder {
    return Builder {
        text: text
    };
}

#[pymethods]
impl Builder{
    pub fn text(&self) -> String {self.text.clone()}
}

impl Builder {

}