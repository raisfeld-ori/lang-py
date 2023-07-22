use std::fmt::Formatter;
use std::ops::Not;
use pyo3::prelude::*;

pub trait Error<T> {
    fn new(description: String) -> T {
        let mut new_args: Vec<String> = Vec::new();
        new_args.push(description);
        return T {
            args: new_args,
        }
    }

}

#[pyclass]
pub struct NotVarError { pub args: Vec<String>}

impl Error<NotVarError> for NotVarError {}