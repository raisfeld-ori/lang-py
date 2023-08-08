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
    pub statements: Vec<(BaseStatement, i32)>,
    pub variables: Vec<(BaseVar, i32)>,
    pub executables: Vec<(BaseExecutable, i32)>,
    pub unknown: Vec<(ShallowParsedLine, i32)>,
}

#[pymethods]
impl BaseOutput{
    #[staticmethod]
    pub fn output_type() ->  AllOutputs {AllOutputs::BaseOutput}
    pub fn statements(&self) -> Vec<(BaseStatement, i32)> {self.statements.clone()}
    pub fn variables(&self) -> Vec<(BaseVar, i32)> {self.variables.clone()}
    pub fn executables(&self) -> Vec<(BaseExecutable, i32)> {self.executables.clone()}
    pub fn unknown(&self) -> Vec<(ShallowParsedLine, i32)> {self.unknown.clone()}
}

pub async fn create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseOutput>{
    let mut threads: Vec<JoinHandle<Option<PyErr>>> = Vec::new();
    let variables: Arc<RwLock<Vec<(BaseVar, i32)>>> = Arc::new(RwLock::new(Vec::new()));
    let statements: Arc<RwLock<Vec<(BaseStatement, i32)>>> = Arc::new(RwLock::new(Vec::new()));
    let executables: Arc<RwLock<Vec<(BaseExecutable, i32)>>> = Arc::new(RwLock::new(Vec::new()));
    let unknown: Arc<RwLock<Vec<(ShallowParsedLine, i32)>>> = Arc::new(RwLock::new(Vec::new()));

    for (i, shallow_line) in shallow_code.iter().enumerate() {
        let variables = variables.clone();
        let statements = statements.clone();
        let executables = executables.clone();
        let unknown = unknown.clone();
        let i_owned = i.clone() as i32;
        let shallow_line = shallow_line.clone();
        threads.push(spawn(async move{
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    let variable = BaseVar::from(shallow_line.to_owned());
                    if variable.is_err() {return Some(variable.unwrap_err())}
                    variables.write().unwrap().push((variable.unwrap(), i_owned));
                }
                CodeType::Statement => {
                    let statement = BaseStatement::from(shallow_line.to_owned());
                    if statement.is_err() {return Some(statement.unwrap_err())}
                    statements.write().unwrap().push((statement.unwrap(), i_owned));
                }
                CodeType::Executable => {
                    let executable = BaseExecutable::from(shallow_line.to_owned());
                    if executable.is_err() {return Some(executable.unwrap_err())}
                    executables.write().unwrap().push((executable.unwrap(), i_owned));
                }
                CodeType::Unknown => {unknown.write().unwrap().push((shallow_line.to_owned(), i_owned));}
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
    let unknown = unknown.read().unwrap();

    return Ok(BaseOutput {
        variables: variables.clone(),
        statements: statements.clone(),
        executables: executables.clone(),
        unknown: unknown.clone(),
        }
    )
}