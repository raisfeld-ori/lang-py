use pyo3::prelude::*;
use crate::extras::errors::{HandledError, NotOperationError};

#[pyclass]#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Operations{
    Addition, Subtraction, Multiplication,
    Division, Modulus, Exponentiation, Floor,
    LessThan, GreaterThan, LessThanOrEqual,
    GreaterThanOrEqual, EqualTo, NotEqual,
    BitwiseAnd, BitwiseOr, BitwiseXor,
    BitwiseNot, BitwiseRightShift,
    BitwiseLeftShift, And, Or, Not,
    Inside, NotIn, Identity, NotIdentity
}
// +, -, *, /, %, **, //, <, >, <=, >=, ==, !=, =, +=, -=, *=, /=, %=, **=, //=, &, |, ^, ~, >>, <<, and, or, not, in, not in, is, is not
impl Operations{
    pub fn get(character: String) -> PyResult<Operations> {
            return match character.as_str() {
                "+" => Ok(Operations::Addition),
                "-" => Ok(Operations::Subtraction),
                "/" => Ok(Operations::Division),
                "&" => Ok(Operations::Modulus),
                "//" => Ok(Operations::Floor),
                _ => Err(NotOperationError ("".to_string(), Some("".to_string())).to_pyerr()),
            }
    }
}