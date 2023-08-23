use crate::python_std::std_types::*;
use pyo3::prelude::*;
use crate::extras::outputs::BaseModule;
use crate::parsing::base_parser::*;
use crate::extras::errors::{HandledError, NotOperationError};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Variable {
    name: String,
    annotation: PythonType,
    value_type: PythonType,
}


#[pyclass]#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct AbstractSyntaxTree{
    pub left: BaseExecutable,
    pub operation: Operations,
    pub right: BaseExecutable,
}
impl AbstractSyntaxTree {
    pub fn from(line: String) -> Option<AbstractSyntaxTree> {
        let mut first_operation = None;
        let mut first_index = usize::MAX;

        for &operation in OPERATION_NOTATIONS.iter() {
            if let Some(index) = line.find(operation) {
                if index < first_index {
                    first_operation = Some(operation);
                    first_index = index;
                }
            }
        }
        if first_operation.is_none() {return None;}
        let left = &line[0..first_index.clone()];
        let right = &line[first_index.clone()..];
        println!("left: {}, right: {}", left, right);

        return None;

    }
}

#[pyclass]#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Operations{
    Addition, Subtraction, Multiplication,
    Division, Modulus, Exponentiation, Floor,
    LessThan, GreaterThan, LessThanOrEqual,
    GreaterThanOrEqual, EqualTo, NotEqual,
    BitwiseAnd, BitwiseOr, BitwiseXor,
    BitwiseNot, BitwiseRightShift,
    BitwiseLeftShift, And, Or, Not,
    In, NotIn, Is, NotIs
}

impl Operations{
    pub fn get(character: String) -> PyResult<Operations> {
            return match character.as_str() {
                "+" => Ok(Operations::Addition),
                "-" => Ok(Operations::Subtraction),
                "/" => Ok(Operations::Division),
                "%" => Ok(Operations::Modulus),
                "//" => Ok(Operations::Floor),
                ">" => Ok(Operations::GreaterThan),
                "<" => Ok(Operations::LessThan),
                "<=" => Ok(Operations::LessThanOrEqual),
                ">=" => Ok(Operations::GreaterThanOrEqual),
                "==" => Ok(Operations::NotEqual),
                "!=" => Ok(Operations::NotEqual),
                "&" => Ok(Operations::BitwiseAnd),
                "|" => Ok(Operations::BitwiseOr),
                "^" => Ok(Operations::BitwiseXor),
                "~" => Ok(Operations::BitwiseNot),
                ">>" => Ok(Operations::BitwiseLeftShift),
                "<<" => Ok(Operations::BitwiseRightShift),
                " and " => Ok(Operations::And),
                " or " => Ok(Operations::Or),
                " not " => Ok(Operations::Not),
                " in " => Ok(Operations::In),
                " not in " => Ok(Operations::NotIn),
                " is " => Ok(Operations::Is),
                " not is " => Ok(Operations::NotIs),
                _ => Err(NotOperationError ("".to_string(), Some("".to_string())).to_pyerr()),
            }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Executable {
    pub components: Vec<Component>,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Component{
    pub actual_line: String,
    pub return_type: Option<Type>,
    pub ast: Option<AbstractSyntaxTree>
}
