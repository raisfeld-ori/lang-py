/*
outputs:
the structs that store the results, so that they can be handled from the python side.
 */

use pyo3::prelude::*;
use std::sync::RwLock;
use crate::base_parser::*;
use tokio::task::{JoinHandle, spawn};
use std::sync::Arc;

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

pub async fn create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseOutput>{
    let mut threads: Vec<JoinHandle<Option<PyErr>>> = Vec::new();
    let variables: Arc<RwLock<Vec<BaseVar>>> = Arc::new(RwLock::new(Vec::new()));
    let statements: Arc<RwLock<Vec<BaseStatement>>> = Arc::new(RwLock::new(Vec::new()));
    let executables: Arc<RwLock<Vec<BaseExecutable>>> = Arc::new(RwLock::new(Vec::new()));
    let unknowns: Arc<RwLock<Vec<Unknown>>> = Arc::new(RwLock::new(Vec::new()));

    for shallow_line in shallow_code.iter() {
        let variables = variables.clone();
        let statements = statements.clone();
        let executables = executables.clone();
        let unknowns = unknowns.clone();
        let shallow_line = shallow_line.clone();
        threads.push(spawn(async move{
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    let variable = BaseVar::from(shallow_line.to_owned());
                    if variable.is_err() {return Some(variable.unwrap_err())}
                    variables.write().unwrap().push(variable.unwrap());
                }
                CodeType::Statement => {
                    let statement = BaseStatement::from(shallow_line.to_owned());
                    if statement.is_err() {return Some(statement.unwrap_err())}
                    statements.write().unwrap().push(statement.unwrap());
                }
                CodeType::Executable => {
                    let executable = BaseExecutable::from(shallow_line.to_owned());
                    if executable.is_err() {return Some(executable.unwrap_err())}
                    executables.write().unwrap().push(executable.unwrap());
                }
                CodeType::Unknown => {
                    let unknown = Unknown::from(shallow_line.to_owned());
                    if unknown.is_err() {return Some(unknown.unwrap_err())}
                    unknowns.write().unwrap().push(unknown.unwrap());
                }
            };
            return None;
        }));
    }

    for thread in threads {
        let error = thread.await.unwrap();
        if error.is_some() {return Err(error.unwrap())}
    }


    let variables = variables.read().unwrap();
    let statements = statements.read().unwrap();
    let executables = executables.read().unwrap();
    let unknowns = unknowns.read().unwrap();

    return Ok(BaseOutput {
        variables: variables.clone(),
        statements: statements.clone(),
        executables: executables.clone(),
        unknown: unknowns.clone(),
        shallow_code: shallow_code.clone(),
        }
    )
}