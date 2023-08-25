use pyo3::prelude::*;
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

}

impl Structure {
    pub fn empty() -> Structure {
        return Structure {

        }
    }
    pub fn std() {

    }
}