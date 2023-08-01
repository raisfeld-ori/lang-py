/*
outputs:
the structs that store the results, so that they can be handled from the python side.
 */

use std::env::var;
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
    let (var_s,mut var_r) = mpsc::channel();
    let (state_s, state_r) = mpsc::channel();
    let (exe_s, exe_r) = mpsc::channel();
    let (unknown_s, unknown_r) = mpsc::channel();

    for shallow_line in shallow_code {
        let var_s = var_s.clone().to_owned();
        let state_s = state_s.clone().to_owned();
        let exe_s = exe_s.clone().to_owned();
        let unknown_s = unknown_s.clone().to_owned();
        threads.push(spawn(async move{
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    var_s.clone().send(BaseVar::from(shallow_line)).unwrap();
                }
                CodeType::Statement => {
                    state_s.send(BaseStatement::from(shallow_line)).unwrap();
                }
                CodeType::Executable => {
                    exe_s.send(BaseExecutable::from(shallow_line)).unwrap();
                }
                CodeType::Unknown => {unknown_s.send(shallow_line).unwrap();}
            };
        }));
    }
    let mut variables: Vec<BaseVar> = Vec::new();
    let mut statements: Vec<BaseStatement> = Vec::new();
    let mut executables: Vec<BaseExecutable> = Vec::new();
    let mut unknown: Vec<ShallowParsedLine> = Vec::new();
    variables.push(var_r.recv().unwrap().unwrap());
    statements.push(state_r.recv().unwrap().unwrap());
    executables.push(exe_r.recv().unwrap().unwrap());
    unknown.push(unknown_r.recv().unwrap());

    return Ok(BaseOutput {
        variables: variables,
        statements: statements,
        executables: executables,
        unknown: unknown,
        }
    )
}