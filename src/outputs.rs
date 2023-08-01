/*
outputs:
the structs that store the results, so that they can be handled from the python side.
 */

use std::io::stderr;
use std::ops::Deref;
use pyo3::exceptions::{PyBaseException, PyException};
use pyo3::prelude::*;
use std::sync::mpsc;
use crate::base_parser::*;
use crate::errors::*;
use tokio::task::{JoinHandle, spawn};

#[pyclass]
pub enum AllOutputs{
    BaseOutput,
}

#[derive(Clone)]
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
    let (var_s, var_r) = mpsc::channel();
    let (state_s, state_r) = mpsc::channel();
    let (exe_s, exe_r) = mpsc::channel();
    let (unknown_s, unknown_r) = mpsc::channel();

    for shallow_line in shallow_code {
        
        threads.push(spawn(async move{
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    var_s.send(BaseVar::from(shallow_line));
                }
                CodeType::Statement => {
                    state_s.send(BaseStatement::from(shallow_line));
                }
                CodeType::Executable => {
                    exe_s.send(BaseExecutable::from(shallow_line));
                }
                CodeType::Unknown => {unknown_s.send(shallow_line);}
            };
        }));
    }
    let variables: Vec<BaseVar> = Vec::new();
    let statements: Vec<BaseStatement> = Vec::new();
    let executables: Vec<BaseExecutable> = Vec::new();
    let unknown: Vec<ShallowParsedLine> = Vec::new();

    return Ok(BaseOutput {
        variables: variables,
        statements: statements,
        executables: executables,
        unknown: unknown,
        }
    )
}