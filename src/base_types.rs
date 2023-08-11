use pyo3::PyResult;
use crate::objects::*;

pub struct PyInt(i32);

impl Object for PyInt {
    fn parse(value: String) -> PyResult<Self> {
        let number = value.parse::<i32>();
        if number.is_err() {number.unwrap_err()}

        return Ok(PyInt(number.unwrap()));
    }
    fn try_parse(value: String) -> Option<Self> {
        let number = value.parse::<i32>();
        if number.is_err() {None}

        return Some(PyInt (number.unwrap()))
    }
}