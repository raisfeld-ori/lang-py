/*
outputs:
full of classes that compile information
 */

use pyo3::prelude::*;
use crate::parsing::base_parser::*;


#[pyclass]
pub enum AllOutputs{
    BaseOutput,
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct BaseOutput {
    pub statements: Vec<BaseStatement>,
    pub variables: Vec<BaseVar>,
    pub executables: Vec<BaseExecutable>,
    pub unknown: Vec<Unknown>,
    pub shallow_code: Vec<ShallowParsedLine>,
}

#[pymethods]
impl BaseOutput{
    #[staticmethod]
    pub fn output_type() ->  AllOutputs {AllOutputs::BaseOutput}
    pub fn statements(&self) -> Vec<BaseStatement> {self.statements.clone()}
    pub fn variables(&self) -> Vec<BaseVar> {self.variables.clone()}
    pub fn executables(&self) -> Vec<BaseExecutable> {self.executables.clone()}
    pub fn unknown(&self) -> Vec<Unknown> {self.unknown.clone()}
    pub fn shallow_code(&self) -> Vec<ShallowParsedLine> { self.shallow_code.clone() }
}

#[pyfunction]
pub fn create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseOutput> {
    let mut variables: Vec<BaseVar> = Vec::new();
    let mut statements: Vec<BaseStatement> = Vec::new();
    let mut executables: Vec<BaseExecutable> = Vec::new();
    let mut unknowns: Vec<Unknown> = Vec::new();
    for shallow_line in shallow_code.iter() {
                    match shallow_line.line_code_type {
                CodeType::Variable => {
                    let variable = BaseVar::from(shallow_line.to_owned());
                    if variable.is_err() {return Err(variable.unwrap_err())}
                    variables.push(variable.unwrap());
                }
                CodeType::Statement => {
                    let statement = BaseStatement::from(shallow_line.to_owned());
                    if statement.is_err() {return Err(statement.unwrap_err())}
                    statements.push(statement.unwrap());
                }
                CodeType::Executable => {
                    let executable = BaseExecutable::from(shallow_line.to_owned());
                    if executable.is_err() {return Err(executable.unwrap_err())}
                    executables.push(executable.unwrap());
                }
                CodeType::Unknown => {
                    let unknown = Unknown::from(shallow_line.to_owned());
                    if unknown.is_err() {return Err(unknown.unwrap_err())}
                    unknowns.push(unknown.unwrap());
                }
            };
    }
    return Ok(BaseOutput {
        variables: variables,
        statements: statements,
        executables: executables,
        unknown: unknowns,
        shallow_code: shallow_code,
    })
}