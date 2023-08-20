/*
errors:
errors in rust take a lot of space, so i just added them all to this file,
this way it's far easier to find specific errors.
 */

use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use pyo3::PyErr;


pub trait HandledError {
    fn to_pyerr(&self) -> PyErr;
}

// error in parsing BaseVar
#[pyclass]#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NotVarError (pub String,pub Option<String>);
impl HandledError for NotVarError{
    fn to_pyerr(&self) -> PyErr  {
        return PyErr::new::<PyBaseException, String>(
            format!("NotVarError: {}\nsuggestion: {}", self.0, self.1.clone().unwrap_or("()".to_string()))
        );
    }
}

// error in parsing BaseStatement
#[pyclass]#[derive(Debug, Clone)] pub struct NotStatementError (pub String, pub Option<String>);

impl HandledError for NotStatementError{
    fn to_pyerr(&self) -> PyErr  {
        return PyErr::new::<PyBaseException, String>(
            format!("NotStatementError: {}\nsuggestion: {}", self.0, self.1.clone().unwrap_or("()".to_string()))
        );
    }
}

#[pyclass]#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct FailedOutputError(pub String, pub Option<String>);

impl HandledError for FailedOutputError{
    fn to_pyerr(&self) -> PyErr  {
        return PyErr::new::<PyBaseException, String>(
            format!("FailedOutputError: {}\nsuggestion: {}", self.0, self.1.clone().unwrap_or("none".to_string()))
        );
    }
}

#[pyclass]#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct NotClassError(pub String, pub Option<String>);

impl HandledError for NotClassError {
    fn to_pyerr(&self) -> PyErr  {
        return PyErr::new::<PyBaseException, String>(
            format!("NotClassError: {}\nsuggestion: {}", self.0, self.1.clone().unwrap_or("()".to_string()))
        );
    }
}