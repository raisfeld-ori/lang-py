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
    pub unknown: Vec<ShallowParsedLine>,
}

#[pymethods]
impl BaseOutput{
    #[staticmethod]
    pub fn output_type() ->  AllOutputs {AllOutputs::BaseOutput}
    pub fn statements(&self) -> Vec<BaseStatement> {self.statements.clone()}
    pub fn variables(&self) -> Vec<BaseVar> {self.variables.clone()}
    pub fn executables(&self) -> Vec<BaseExecutable> {self.executables.clone()}
    pub fn unknown(&self) -> Vec<ShallowParsedLine> {self.unknown.clone()}
}

pub async fn create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseOutput>{
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let variables: Arc<RwLock<Vec<BaseVar>>> = Arc::new(RwLock::new(Vec::new()));
    let statements: Arc<RwLock<Vec<BaseStatement>>> = Arc::new(RwLock::new(Vec::new()));
    let executables: Arc<RwLock<Vec<BaseExecutable>>> = Arc::new(RwLock::new(Vec::new()));
    let unknown: Arc<RwLock<Vec<ShallowParsedLine>>> = Arc::new(RwLock::new(Vec::new()));

    for shallow_line in shallow_code {
        let variables = variables.clone();
        let statements = statements.clone();
        let executables = executables.clone();
        let unknown = unknown.clone();
        threads.push(spawn(async move{
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    variables.write().unwrap().push(BaseVar::from(shallow_line).unwrap());
                }
                CodeType::Statement => {
                    statements.write().unwrap().push(BaseStatement::from(shallow_line).unwrap());
                }
                CodeType::Executable => {
                    executables.write().unwrap().push(BaseExecutable::from(shallow_line).unwrap());
                }
                CodeType::Unknown => {unknown.write().unwrap().push(shallow_line);}
            };
        }));
    }

    for thread in threads {
        thread.await.unwrap();
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