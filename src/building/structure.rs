use pyo3::prelude::*;
use crate::extras::errors::{FormattingError, HandledError};
//use crate::parsing::type_parsing::Operations;
//use crate::parsing::base_parser::OPERATION_NOTATIONS;
//use crate::parsing::object_parsing::StatementType;

/*
pub static STATEMENTS: [&str; 14] = ["if", "else", "elif",
    "for", "def", "async", "try", "except",
    "finally", "while", "from",
    "class", "with", "global"];
 */

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Structure{
    pub for_loop: String,
    pub if_statement: String,
    pub else_statement: String,
    pub elif_statement: String,
    pub while_loop: String,
    pub with: String,
    pub define: String,
    pub class: String,
    pub global: String,
    pub variable: String,
    pub executable: String,
}

impl Structure {
    pub fn empty() -> Structure {
        return Structure {
            for_loop: String::new(),
            if_statement: String::new(),
            else_statement: String::new(),
            elif_statement: String::new(),
            while_loop: String::new(),
            with: String::new(),
            define: String::new(),
            class: String::new(),
            global: String::new(),
            variable: String::new(),
            executable: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct Format {
    pub line: String,
    pub value: Option<Vec<String>>,
    pub replace: String,
}

#[pymethods]
impl Format {
    pub fn apply(&mut self) -> PyResult<String> {
        let values = self.value.clone().unwrap_or(Vec::new());
        let mut values = values.iter();
        let mut error: Option<&str> = None;
        let result: String = self.line.split(self.replace.as_str()).map(
            |slice|
            if let Some(value) = values.next() {
                format!("{}{}", slice, value)
            }
            else {
                error = Some(slice);
                format!("{}", slice)
            }
        ).collect::<String>();
        if error.is_some(){
            return Err(FormattingError (
                format!("the line {} did not have a value to insert", error.unwrap()),None
                ).to_pyerr());
        }
        else{
            return Ok(result);
        }
    }
}