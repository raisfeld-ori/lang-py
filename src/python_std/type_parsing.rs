use crate::python_std::std_types::*;
use pyo3::prelude::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Variable {
    name: String,
    annotation: PythonType,
    value_type: PythonType,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Executable {

}

impl Executable {
    //pub fn from(base_executable: BaseExecutable, module: BaseModule) {

    //}
}