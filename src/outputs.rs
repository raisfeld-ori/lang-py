/*
outputs:
the structs that store the results, so that they can be handled from the python side.
 */

use std::env::var;
use pyo3::prelude::*;
use crate::base_parser::*;
use crate::errors::*;


#[derive(Clone)]
#[pyclass]
pub struct BaseOutput {
    pub statements: Vec<BaseStatement>,
    pub statement_errors: Vec<NotStatementError>,
    pub variables: Vec<BaseVar>,
    pub variable_errors: Vec<NotVarError>,
    pub unknown: Vec<String>,
}

impl BaseOutput{
    pub fn from(shallow_code: Vec<ShallowParsedLine>) -> BaseOutput{
        let mut variables: Vec<BaseVar> = Vec::new();
        let mut variable_errors: Vec<NotVarError> = Vec::new();
        let mut statements: Vec<BaseStatement> = Vec::new();
        let mut statement_errors: Vec<NotStatementError> = Vec::new();
        let mut unknown: Vec<String> = Vec::new();
        for shallow_line in shallow_code {
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    let output = BaseVar::from(shallow_line);
                    match output {
                        Err(error) => variable_errors.push(error),
                        Ok(var) => variables.push(var),
                    }
                }
                CodeType::Statement => {
                    let output = BaseStatement::from(shallow_line);
                    match output {
                        Err(error) => statement_errors.push(error),
                        Ok(var) => statements.push(var),
                    }
                }
                _ => {unknown.push(shallow_line.actual_line)}
            }
        }
        return BaseOutput {
            variables: variables,
            variable_errors: variable_errors,
            statements: statements,
            statement_errors: statement_errors,
            unknown: unknown,
        }
    }
}