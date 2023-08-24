/*
outputs:
full of classes that compile information
 */

use std::cmp::Ordering;
use pyo3::prelude::*;
use crate::parsing::base_parser::*;
use crate::parsing::object_parsing::{BaseMethod, BaseObject, StatementType};
use std::collections::HashMap;
use std::ops::Deref;
use crate::python_std::std_types::{PythonType, Type};

#[pyclass]
pub enum AllOutputs{
    BaseCode, BaseModule
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct BaseCode {
    pub statements: Vec<BaseStatement>,
    pub variables: Vec<BaseVar>,
    pub executables: Vec<BaseExecutable>,
    pub unknown: Vec<Unknown>,
    pub shallow_code: Vec<ShallowParsedLine>,
    pub imports: Vec<BaseStatement>,
}

#[pymethods]
impl BaseCode{
    #[staticmethod]
    pub fn output_type() ->  AllOutputs {AllOutputs::BaseCode}
    pub fn statements(&self) -> Vec<BaseStatement> {self.statements.clone()}
    pub fn variables(&self) -> Vec<BaseVar> {self.variables.clone()}
    pub fn executables(&self) -> Vec<BaseExecutable> {self.executables.clone()}
    pub fn unknown(&self) -> Vec<Unknown> {self.unknown.clone()}
    pub fn shallow_code(&self) -> Vec<ShallowParsedLine> { self.shallow_code.clone() }
    pub fn imports(&self) -> Vec<BaseStatement> {self.imports.clone()}
}

#[pyfunction]
pub fn create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseCode> {
    let mut variables: Vec<BaseVar> = Vec::new();
    let mut statements: Vec<BaseStatement> = Vec::new();
    let mut executables: Vec<BaseExecutable> = Vec::new();
    let mut unknowns: Vec<Unknown> = Vec::new();
    let mut imports: Vec<BaseStatement> = Vec::new();
    for shallow_line in shallow_code.iter() {
                    match shallow_line.line_code_type {
                CodeType::Variable => {
                    let variable = BaseVar::from(shallow_line.to_owned());
                    if variable.is_err() {return Err(variable.unwrap_err())}
                    variables.push(variable.unwrap());
                }
                CodeType::Statement => {
                    let statement = BaseStatement::from(shallow_line.to_owned());
                    if statement.is_err() {return Err(statement.unwrap_err())}
                    let statement = statement.unwrap();

                    if statement.statement_type == StatementType::Import {
                        imports.push(statement.clone())
                    }
                    statements.push(statement);
                }
                CodeType::Executable => {
                    let executable = BaseExecutable::from(shallow_line.to_owned());
                    if executable.is_err() {return Err(executable.unwrap_err())}
                    executables.push(executable.unwrap());
                }
                CodeType::Unknown => {
                    let unknown = Unknown::from(shallow_line.to_owned());
                    if unknown.is_err() {return Err(unknown.unwrap_err())}
                    unknowns.push(unknown.unwrap());
                }
            };
    }
    return Ok(BaseCode {
        variables: variables,
        statements: statements,
        executables: executables,
        unknown: unknowns,
        shallow_code: shallow_code,
        imports: imports,
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct BaseModule {
    pub actual_text: String,
    pub name: String,
    pub code: BaseCode,
    pub objects: Vec<BaseObject>,
    pub methods: Vec<BaseMethod>,
    pub all: HashMap<String, Type>,
}
// HashMap doesn't implement PartialOrd ):
impl PartialOrd for BaseModule {
    fn ge(&self, other: &Self) -> bool {
        return self.name.ge(&other.name)
            && self.code.ge(&other.code)
            && self.objects.ge(&other.objects)
            && self.methods.ge(&other.methods);
    }
    fn gt(&self, other: &Self) -> bool {
        return self.name.gt(&other.name)
            && self.code.gt(&other.code)
            && self.objects.gt(&other.objects)
            && self.methods.gt(&other.methods);
    }
    fn le(&self, other: &Self) -> bool {
        return self.name.le(&other.name)
            && self.code.le(&other.code)
            && self.objects.le(&other.objects)
            && self.methods.le(&other.methods);
    }
    fn lt(&self, other: &Self) -> bool {
        return self.name.lt(&other.name)
            && self.code.lt(&other.code)
            && self.objects.lt(&other.objects)
            && self.methods.lt(&other.methods);
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.name.partial_cmp(&other.name)
            .and_then(|o| if o == Ordering::Equal { self.code.partial_cmp(&other.code) } else { Some(o) })
            .and_then(|o| if o == Ordering::Equal { self.objects.partial_cmp(&other.objects) } else { Some(o) })
            .and_then(|o| if o == Ordering::Equal { self.methods.partial_cmp(&other.methods) } else { Some(o) }) {
                Some(Ordering::Greater) | Some(Ordering::Equal) => Some(Ordering::Equal),
                Some(Ordering::Less) => Some(Ordering::Less),
                None => None,
        }
    }
}

#[pymethods]
impl BaseModule {
    pub fn code(&self) -> BaseCode {self.code.clone()}
    pub fn objects(&self) -> Vec<BaseObject> {self.objects.clone()}
    pub fn methods(&self) -> Vec<BaseMethod> {self.methods.clone()}
    pub fn name(&self) -> String {self.name.clone()}
    pub fn all_names(&self) -> Vec<String> {
        (*self
            .all
            .clone()
            .keys()
            .into_iter()
            .map(|key| key.deref().to_string())
            .collect::<Vec<String>>()
        ).to_vec()
    }
    pub fn all_classes(&self) -> Vec<PythonType> {
        (*self
            .all
            .clone()
            .values()
            .into_iter()
            .map(|key| PythonType (key.clone()))
            .collect::<Vec<PythonType>>()
        ).to_vec()
    }
}