use crate::python_std::std_types::*;
use pyo3::prelude::*;
use crate::extras::outputs::BaseModule;
use crate::parsing::base_parser::*;

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
    pub fn from(base_executable: BaseExecutable, module: BaseModule) {
        let mut scope: Type = Type::Module(module);
        for component in base_executable.components {
            match scope.clone() {
                Type::Module(scope) => {

                }
                Type::Method(method) => {

                }
                Type::Object(object) => {

                }
                Type::Standard(std_type) => {

                }
            }
        }
    }
}