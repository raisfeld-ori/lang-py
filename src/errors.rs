/*
errors:
errors in rust take a lot of space, so i just added them all to this file,
this way it's far easier to find specific errors.
 */

use pyo3::prelude::*;
use pyo3::PyErr;


pub trait HandledError {
    fn mark(&mut self, line: String, pos: i32);
    fn is_err() -> bool {true}
    fn to_pyerr(&self) -> PyErr;
}

// error in parsing BaseVar
#[pyclass]#[derive(Debug, Clone)]pub struct NotVarError (pub String,pub Vec<(String, i32)>);
impl From<&str> for NotVarError {
    fn from(value: &str) -> Self {
        return NotVarError(format!("NotVarError: {}", value),
                           Vec::new()); } }
impl From<String> for NotVarError {
    fn from(value: String) -> Self {
        return NotVarError(format!("NotVarError: {}", value),
                           Vec::new()); } }
impl HandledError for NotVarError{
    fn mark(&mut self, line: String, pos: i32) { self.1.push((line, pos)) }
    fn to_pyerr(&self) -> PyErr  {
        let mut traceback: String = String::new();
        for (tr_line, tr_pos) in &self.1{
            traceback.push_str(format!("at line {}: {}", tr_pos, tr_line).as_str())
        }
        traceback.push_str(self.0.to_string().as_str());

        return PyErr::new::<PyAny, String>(traceback);
    }
}

// error in parsing BaseStatement
#[pyclass]#[derive(Debug, Clone)] pub struct NotStatementError (pub String, pub Vec<(String, i32)>);
impl From<&str> for NotStatementError {
    fn from(value: &str) -> Self {
        return NotStatementError
            (format!("NotStatementError: {}", value),
                Vec::new());
            }
}
impl From<String> for NotStatementError {
    fn from(value: String) -> Self {
        return NotStatementError
            (format!("NotStatementError: {}", value),
                Vec::new());
            }
}

impl HandledError for NotStatementError{
    fn mark(&mut self, line: String, pos: i32) { self.1.push((line, pos)); }
    fn to_pyerr(&self) -> PyErr  {
        let mut traceback: String = String::new();
        for (tr_line, tr_pos) in &self.1{
            traceback.push_str(format!("at line {}: {}", tr_pos, tr_line).as_str())
        }
        traceback.push_str(self.0.to_string().as_str());

        return PyErr::new::<PyAny, String>(traceback);
    }
}

#[pyclass]#[derive(Clone, Debug)]pub struct FailedOutputError(pub String, pub Vec<(String, i32)>);
impl From<&str> for FailedOutputError {
    fn from(value: &str) -> Self {
        return FailedOutputError
            (format!("FailedOutputError: {}", value),
                Vec::new());
            }
}
impl From<String> for FailedOutputError {
    fn from(value: String) -> Self {
        return FailedOutputError
            (format!("FailedOutputError: {}", value),
                Vec::new());
            }
}
impl HandledError for FailedOutputError{
    fn mark(&mut self, line: String, pos: i32) { self.1.push((line, pos)); }
    fn to_pyerr(&self) -> PyErr  {
        let mut traceback: String = String::new();
        for (tr_line, tr_pos) in &self.1{
            traceback.push_str(format!("at line {}: {}", tr_pos, tr_line).as_str())
        }
        traceback.push_str(self.0.to_string().as_str());

        return PyErr::new::<PyAny, String>(traceback);
    }
}