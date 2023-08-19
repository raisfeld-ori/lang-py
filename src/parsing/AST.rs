use pyo3::prelude::*;
use crate::extras::errors::{HandledError, WrongOperationError};

pub trait Operation {
    type First;
    type Second;
    fn parse(input: String) -> PyResult<Self>;
}

#[pyclass]
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Addition(pub String, pub String);

impl Operation for Addition{
    type First = String;
    type Second = String;
    fn parse(input: String) -> PyResult<Self> {
        if !input.contains("+") {
            return Err(WrongOperationError (
                "the operation is not addition".to_string(),
                Some("".to_string())).to_pyerr());
        }
        return Ok(Addition ("".to_string(), "".to_string()))
    }
}