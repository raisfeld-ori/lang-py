/*
errors:
errors in rust take a lot of space, so i just added them all to this file,
this way it's far easier to find specific errors.
 */

use pyo3::prelude::*;
use std::fmt::{Display, Formatter};

// error in parsing BaseVar
#[pyclass]#[derive(Debug, Clone)]pub struct NotVarError (pub String);
#[pymethods]impl NotVarError { pub fn to_string(&self) -> String {return format!("{}: {}", "NotVarError", self.0)} }
impl Display for NotVarError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}: {}", "NotVarError", self.0) }
}

// error in parsing BaseStatement
#[pyclass]#[derive(Debug, Clone)] pub struct NotStatementError (pub String);
#[pymethods] impl NotStatementError {pub fn  to_string(&self) -> String {return format!("{}: {}", "NotStatementError", self.0)}}
impl Display for NotStatementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {write!(f, "{}: {}", "NotStatementError", self.0)}
}

