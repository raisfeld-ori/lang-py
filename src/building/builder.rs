use pyo3::prelude::*;
use crate::building::structure::*;
use crate::extras::outputs::BaseModule;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Builder{
    pub text: String,
    pub module: BaseModule,
    pub structure: Structure,
}

#[pymethods]
impl Builder {
    pub fn text(&self) -> String {self.text.clone()}

    #[staticmethod]
    pub fn with_std(module: BaseModule) -> Builder {
    return Builder {
        text: module.actual_text.clone(),
        module: module,
        structure: Structure::empty(),
    };
}
}