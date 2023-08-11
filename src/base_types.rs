use pyo3::prelude::*;
use tokio::task;
use tokio::runtime::Builder;
use std::thread;
use crate::base_parser::*;
use crate::errors::{HandledError, NotStatementError};

#[pyfunction]
pub fn get_methods(statements: Vec<BaseStatement>) -> PyResult<Vec<BaseMethod>> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let thread: thread::JoinHandle<PyResult<Vec<BaseMethod>>>= thread::spawn(move ||{
            runner.block_on(async move {
                let mut threads: Vec<task::JoinHandle<Option<PyResult<BaseMethod>>>> = Vec::new();
                for statement in statements{
                    let statement_owned = statement.to_owned();
                    threads.push(task::spawn(async move {
                        if statement_owned.statement_type == StatementType::Def{
                            Some(BaseMethod::from(statement_owned))
                        }
                        else {
                            None
                        }
                    }));
                }
                let mut base_methods: Vec<BaseMethod> = Vec::new();
                for thread in threads{
                    let output = thread.await.unwrap();
                    if output.is_none() {continue;}
                    let output = output.unwrap();
                    if output.is_err() {return Err(output.unwrap_err());}
                    else {base_methods.push(output.unwrap());}
                }
                return Ok(base_methods);

            })
        });
    return thread.join().unwrap();
}


// every type of statement
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[pyclass]
pub enum StatementType {
    For, While, Import, Try, Except, If,
    Else, Elif, With, Class, Finally, Def,
    From
}

impl StatementType{
    pub fn from(word: &str) -> PyResult<StatementType> {
        match word {
            "if" => {Ok(StatementType::If)}
            "while" => {Ok(StatementType::While)}
            "else" => {Ok(StatementType::Else)}
            "elif" => {Ok(StatementType::Elif)}
            "for" => {Ok(StatementType::For)}
            "with" => {Ok(StatementType::With)}
            "class" => {Ok(StatementType::Class)}
            "try" => {Ok(StatementType::Try)}
            "except" => {Ok(StatementType::Except)}
            "finally" => {Ok(StatementType::Finally)}
            "def" => {Ok(StatementType::Def)}
            "import" => {Ok(StatementType::Import)}
            "from" => {Ok(StatementType::From)}
            _ => {Err(NotStatementError::from(
                format!("could not parse the statement type {}", word)).to_pyerr())}
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[pyclass]
pub struct BaseMethod{
    name: String,
    input: Vec<String>,
    outputs: Vec<String>,
    derivatives: Vec<String>,
    lines: Vec<String>,
}

#[pymethods]
impl BaseMethod{
    pub fn name(&self) -> String {self.name.clone()}
    pub fn input(&self) -> Vec<String> {self.input.clone()}
    pub fn output(&self) -> Vec<String> {self.outputs.clone()}
    pub fn derivatives(&self) -> Vec<String> {self.derivatives.clone()}
    pub fn lines(&self) -> Vec<String> {self.lines.clone()}
}

impl BaseMethod{
    pub fn from(line: BaseStatement) -> PyResult<Self> {
        if line.statement_type != StatementType::Def {
                        return Err(
                            NotStatementError::from(format!("expected a function, got an {:?} statement", line.statement_type.clone()))
                                .to_pyerr())
        }

        if !line.actual_line.actual_line.contains('(') || !line.actual_line.actual_line.contains(')') {
            return Err(NotStatementError::from(format!("line does not contain variables ( missing '(' )\
\n\n(from the line: {})", line.actual_line.actual_line)).to_pyerr())
        }

        let mut name: String = String::new();
        let mut input: Vec<String> = Vec::new();
        let mut output: Vec<String>  = Vec::new();
        let derivatives: Vec<String> = Vec::new();

        let mut current: String = String::new();
        for (i, char) in line.actual_line.actual_line.chars().enumerate(){
            if line.is_async && i > 12 && i < line.actual_line.actual_line.find('(').unwrap() {
                if char == '(' {continue}
                name.push(char);
            }
            else if i > 7 && i < line.actual_line.actual_line.find('(').unwrap() {
                if char == '(' {break}
                name.push(char);
            }
            else if i > line.actual_line.actual_line.find('(').unwrap() && i < line.actual_line.actual_line.find(')').unwrap() + 1{
                if char == ',' || char == ')' {input.push(current.clone()); current = String::new();continue;}
                current.push(char);
            }
            else if i == line.actual_line.actual_line.find(")").unwrap() {current = String::new();}
            else if i > line.actual_line.actual_line.find(')').unwrap() {
                if !line.actual_line.actual_line.contains("->") {output.push("None".to_string());break;}
                else {
                    if i > line.actual_line.actual_line.find("->").unwrap() + 1 {
                        if char == ' ' || char == ':' {continue}
                        current.push(char)
                    }
                }
            }
        }
        if current != String::new() {output.push(current)}

        return Ok(BaseMethod {
            name: name,
            input: input,
            outputs: output,
            derivatives: derivatives,
            lines: Vec::new(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[pyclass]
pub struct Object{
    name: String,
    inheritance: Vec<String>,
}

