use pyo3::prelude::*;
use crate::parsing::base_parser::*;
use crate::extras::errors::{HandledError, NotClassError, NotStatementError};


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
            _ => {Err(NotStatementError(
                format!("could not parse the statement type {}", word), None).to_pyerr())}
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[pyclass]
pub struct Method{
    name: String,
    input: Vec<String>,
    outputs: Vec<String>,
    derivatives: Vec<String>,
    lines: Vec<ShallowParsedLine>,
    actual_line: BaseStatement,
}

#[pymethods]
impl Method{
    pub fn name(&self) -> String {self.name.clone()}
    pub fn input(&self) -> Vec<String> {self.input.clone()}
    pub fn output(&self) -> Vec<String> {self.outputs.clone()}
    pub fn derivatives(&self) -> Vec<String> {self.derivatives.clone()}
    pub fn lines(&self) -> Vec<ShallowParsedLine> {self.lines.clone()}
    pub fn actual_line(&self) -> BaseStatement {self.actual_line.clone()}
}

impl Method{
    pub fn from(line: BaseStatement, mut all_lines: Vec<ShallowParsedLine>) -> PyResult<Self> {
        if line.statement_type != StatementType::Def {
                        return Err(
NotStatementError(
    format!("expected a function, got an {:?} statement", line.statement_type.clone()), None)
    .to_pyerr())
        }

        if !line.actual_line.actual_line.contains('(') || !line.actual_line.actual_line.contains(')') {
            return Err(NotStatementError (format!("line does not have input (missing '(')\
\n\n(from the line: {})", line.actual_line.actual_line), None).to_pyerr())
        }

        let mut name: String = String::new();
        let mut input: Vec<String> = Vec::new();
        let mut output: Vec<String>  = Vec::new();

        let mut current: String = String::new();
        for (i, char) in line.actual_line.actual_line.chars().enumerate(){
            if i > line.actual_line.actual_line.find("def").unwrap() + 3
                && i < line.actual_line.actual_line.find('(').unwrap() {
                if char == '(' {break}
                name.push(char);
            }
            else if i > line.actual_line.actual_line.find('(').unwrap()
                && i < line.actual_line.actual_line.find(')').unwrap() + 1{
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
        let mut derivatives: Vec<String> = Vec::new();
        let mut lines: Vec<ShallowParsedLine> = Vec::new();
        all_lines.sort_by_key(|line| line.position);
        for other_line in all_lines.iter() {
            if other_line.position > line.actual_line.position {
                if other_line.all_spaces <= line.actual_line.all_spaces
                    && other_line.actual_line.replace(" ", "") != ""{break;}
                lines.push(other_line.clone());
            }
            else if other_line.actual_line.replace(" ", "").starts_with("@") {
                derivatives.push(other_line.actual_line.clone());
            }
        }

        return Ok(Method {
            name: name,
            input: input,
            outputs: output,
            derivatives: derivatives,
            lines: lines.clone(),
            actual_line: line.clone(),
        })
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Object{
    name: String,
    inheritance: Vec<String>,
    methods: Vec<Method>,
    lines: Vec<ShallowParsedLine>,
}

#[pymethods]
impl Object {
    pub fn name(&self) ->  String {self.name.clone()}
    pub fn inheritance(&self) -> Vec<String> {self.inheritance.clone()}
    pub fn lines(&self) -> Vec<ShallowParsedLine> {self.lines.clone()}
    pub fn methods(&self) -> Vec<Method> {self.methods.clone()}
}

impl Object{
    pub fn from(statement: BaseStatement,mut all_lines: Vec<ShallowParsedLine>,mut parsed_methods: Vec<Method>) -> PyResult<Object> {
        if statement.statement_type != StatementType::Class {
            return Err(NotClassError (
                format!("expected a class, found {:?}", statement.statement_type), None)
                .to_pyerr())
        }
        if !statement.actual_line.actual_line.contains("class"){
            return Err(NotClassError (
                format!("the class given does not include 'class'"), None)
                .to_pyerr()
            )
        }

        let mut name: String = String::new();
        let mut inheritance: Vec<String> = Vec::new();

        let mut current: String = String::new();
        for (i, char) in statement.actual_line.actual_line.chars().enumerate() {
            if i > statement.actual_line.actual_line.find("class").unwrap() + 5
            && i < statement.actual_line.actual_line.find("(").unwrap_or(0){
                name.push(char);
            }
            else if i > statement.actual_line.actual_line.find("(").unwrap_or(0)
            && i < statement.actual_line.actual_line.find(")").unwrap_or(0){
                if char == ',' || char == ')' {inheritance.push(current.clone());current.clear();}
                else{
                    current.push(char);
                }
            }
        }
        all_lines.sort_by_key(|line| line.position);
        parsed_methods.sort_by_key(|method| method.actual_line.actual_line.position);
        let mut lines: Vec<ShallowParsedLine> = Vec::new();
        let mut methods: Vec<Method> = Vec::new();
        let mut iter_methods = parsed_methods.iter();
        let mut current_method = iter_methods.next().unwrap();
        for other_line in all_lines.iter() {
            if other_line.position > statement.actual_line.position {

                if other_line.all_spaces <= statement.actual_line.all_spaces
                    && other_line.actual_line.replace(" ", "") != ""{ break }

                if other_line.line_code_type == CodeType::Statement
                    && other_line.position == current_method.actual_line.actual_line.position {
                         methods.push(current_method.clone());
                        current_method = iter_methods.next().unwrap();
                }
                lines.push(other_line.clone());
            }
        }

        return Ok(Object {
            name: name,
            inheritance: inheritance,
            lines: lines,
            methods: methods,
        })
    }
}

#[pyfunction]
pub fn parse_objects(statements: Vec<BaseStatement>, all_lines: Vec<ShallowParsedLine>, methods: Vec<Method>) -> PyResult<Vec<Object>>{
    let mut objects: Vec<Object> = Vec::new();
    for statement in statements {
        if statement.statement_type == StatementType::Class {
            objects.push(Object::from(statement, all_lines.clone(), methods.clone()).unwrap());
        }
    }
    return Ok(objects);
}

#[pyfunction]
pub fn parse_methods(statements: Vec<BaseStatement>, all_lines: Vec<ShallowParsedLine>) -> PyResult<Vec<Method>>{
    let mut objects: Vec<Method> = Vec::new();
    for statement in statements {
        if statement.statement_type == StatementType::Class {
            objects.push(Method::from(statement, all_lines.clone()).unwrap());
        }
    }
    return Ok(objects);
}