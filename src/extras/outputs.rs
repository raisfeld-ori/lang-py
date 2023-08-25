/*
outputs:
full of classes that compile information
 */

use pyo3::prelude::*;
use crate::parsing::base_parser::*;
use crate::parsing::object_parsing::{BaseMethod, BaseObject, StatementType};

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[pyclass]
pub struct BaseModule {
    pub actual_text: String,
    pub name: String,
    pub code: BaseCode,
    pub objects: Vec<BaseObject>,
    pub methods: Vec<BaseMethod>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct PysortLine(pub BaseModuleLine);

#[pymethods]
impl PysortLine{
    pub fn shallow_global(&self) -> ShallowParsedLine {self.0.clone().get_line()}
        pub fn get_executable(&self) -> Option<BaseExecutable>{
        match self.0.clone(){
            BaseModuleLine::Executable(exe) => {return Some(exe.clone());}
            _ => None
        }
    }
    pub fn get_var(&self) -> Option<BaseVar>{
        match self.0.clone(){
            BaseModuleLine::Variable(var) => {return Some(var.clone());}
            _ => None
        }
    }
    pub fn get_unknown(&self) -> Option<Unknown>{
        match self.0.clone(){
            BaseModuleLine::Unknown(unknown) => {return Some(unknown.clone());}
            _ => None
        }
    }
    pub fn get_statement(&self) -> Option<BaseStatement>{
        match self.0.clone(){
            BaseModuleLine::Statement(statement) => {return Some(statement.clone());}
            _ => None
        }
    }
    pub fn get_method(&self) -> Option<BaseMethod>{
        match self.0.clone(){
            BaseModuleLine::Method(meth) => {return Some(meth.clone());}
            _ => None
        }
    }
    pub fn get_object(&self) -> Option<BaseObject>{
        match self.0.clone(){
            BaseModuleLine::Object(obj) => {return Some(obj.clone());}
            _ => None
        }
    }

}

#[pymethods]
impl BaseModule {
    pub fn code(&self) -> BaseCode {self.code.clone()}
    pub fn objects(&self) -> Vec<BaseObject> {self.objects.clone()}
    pub fn methods(&self) -> Vec<BaseMethod> {self.methods.clone()}
    pub fn name(&self) -> String {self.name.clone()}
    pub fn to_pysort(&self) -> Vec<PysortLine> {
        return self.sort().iter().map(|line| PysortLine (line.clone())).collect();
    }
}

impl BaseModule {
    pub fn sort(&self) -> Vec<BaseModuleLine>{
        let mut sorted_module: Vec<(BaseModuleLine, usize)> = Vec::new();
        sorted_module.append(&mut self.code.variables.clone().iter().map(
            |var| (BaseModuleLine::Variable(var.clone()), var.actual_line.position))
                                 .collect::<Vec<(BaseModuleLine, usize)>>());
        sorted_module.append(&mut self.code.executables.clone().iter().map(
            |exe| (BaseModuleLine::Executable(exe.clone()), exe.actual_line.position))
            .collect::<Vec<(BaseModuleLine, usize)>>()
        );
        sorted_module.append(&mut self.code.statements.clone().iter().map(
            |statement| (BaseModuleLine::Statement(statement.clone()), statement.actual_line.position))
            .collect::<Vec<(BaseModuleLine, usize)>>()
        );
        sorted_module.append(&mut self.code.unknown.clone().iter().map(
            |unknown| (BaseModuleLine::Unknown(unknown.clone()), unknown.actual_line.position))
            .collect::<Vec<(BaseModuleLine, usize)>>()
        );
        sorted_module.append(&mut self.objects.clone().iter().map(
            |obj| (BaseModuleLine::Object(obj.clone()), obj.actual_line.actual_line.position))
            .collect::<Vec<(BaseModuleLine, usize)>>()
        );
        sorted_module.append(&mut self.methods.clone().iter().map(
            |method| (BaseModuleLine::Method(method.clone()), method.actual_line.actual_line.position))
            .collect::<Vec<(BaseModuleLine, usize)>>()
        );
        sorted_module.sort_by_key(|line|line.1.clone());
        return sorted_module.iter().map(|line|line.0.clone()).collect();
    }
}
/*
    pub fn code(&self) -> BaseCode {self.code.clone()}
    pub fn objects(&self) -> Vec<BaseObject> {self.objects.clone()}
    pub fn methods(&self) -> Vec<BaseMethod> {self.methods.clone()}
    pub fn name(&self) -> String {self.name.clone()}
    pub fn all_names(&self) -> Vec<String>
 */

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum BaseModuleLine{
    Executable (BaseExecutable),
    Variable (BaseVar),
    Unknown (Unknown),
    Statement (BaseStatement),
    Object (BaseObject),
    Method (BaseMethod)
}

impl BaseModuleLine {
    pub fn get_line(&self) -> ShallowParsedLine{
        match self{
            BaseModuleLine::Unknown(unknown) => unknown.clone().actual_line,
            BaseModuleLine::Statement(statement) => statement.clone().actual_line,
            BaseModuleLine::Executable(exe) => exe.clone().actual_line,
            BaseModuleLine::Object(obj) => obj.clone().actual_line.actual_line,
            BaseModuleLine::Method(method) => method.clone().actual_line.actual_line,
            BaseModuleLine::Variable(var) => var.clone().actual_line,
        }
    }
    pub fn get_executable(&self) -> Option<BaseExecutable>{
        match self{
            BaseModuleLine::Executable(exe) => {return Some(exe.clone());}
            _ => None
        }
    }
    pub fn get_var(&self) -> Option<BaseVar>{
        match self{
            BaseModuleLine::Variable(var) => {return Some(var.clone());}
            _ => None
        }
    }
    pub fn get_unknown(&self) -> Option<Unknown>{
        match self{
            BaseModuleLine::Unknown(unknown) => {return Some(unknown.clone());}
            _ => None
        }
    }
    pub fn get_statement(&self) -> Option<BaseStatement>{
        match self{
            BaseModuleLine::Statement(statement) => {return Some(statement.clone());}
            _ => None
        }
    }
    pub fn get_method(&self) -> Option<BaseMethod>{
        match self{
            BaseModuleLine::Method(meth) => {return Some(meth.clone());}
            _ => None
        }
    }
    pub fn get_object(&self) -> Option<BaseObject>{
        match self{
            BaseModuleLine::Object(obj) => {return Some(obj.clone());}
            _ => None
        }
    }
}