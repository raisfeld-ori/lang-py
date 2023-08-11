/*
base_parser:
does the basic parsing of the code, meaning it parses things like the code structure,
the name and value of a variable etc.
 */
use std::any::Any;
use pyo3::prelude::*;
use crate::errors::*;
use tokio::task::{JoinHandle, spawn};
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::base_types::StatementType;
use PyErr;


static OPERATORS: [char; 3] = ['>', '<', '!'];
static STATEMENTS: [&str; 13] = ["if", "else", "elif",
    "for", "def", "async", "try", "except",
    "finally", "while", "from",
    "class", "with"];

/*
all python of python's code can be categorized into 4 types:
Variable - a line of code with a name and a value, where the name points to the value.
Executable - any line that ends by using a function.
Statement - lines of code which store other lines of code.
Unknown - any line of code that has no effect on the rest of the code, therefore doesn't matter.
 */
#[derive(Debug, Clone)]
#[pyclass]
pub enum CodeType{
    Variable,
    Executable,
    Unknown,
    Statement,
}


// the most basic parsing of the code
#[derive(Debug, Clone)]
#[pyclass]
pub struct ShallowParsedLine{
    pub line_code_type: CodeType,
    pub actual_line: String,
    pub all_spaces: i32,
    pub placement: Option<i32>,
}

#[pymethods]
impl ShallowParsedLine {
    pub fn line_code_type(&self) -> CodeType {self.line_code_type.clone()}
    pub fn actual_line(&self) -> String {self.actual_line.clone()}
    pub fn all_spaces(&self) -> i32 {self.all_spaces.clone()}
    pub fn placement(&self) -> Option<i32> {self.placement.clone()}
}

// part of the from function, but it takes a lot of space, so i added it here
async fn from_parse(i: usize, line: String) -> ShallowParsedLine {
    let mut line_type = CodeType::Unknown;
    if line.ends_with(")") { line_type = CodeType::Executable; }
    let first_word = line.clone().split_whitespace().next().unwrap_or("").replace(":", "");

    if STATEMENTS.contains(&first_word.as_str()) { line_type = CodeType::Statement; }

    if line.contains("=") {
        let first_equation: usize = line.find("=").unwrap();
        if line.chars().nth(first_equation + 1).unwrap() != '=' {
            if !OPERATORS.contains(&line.chars().nth(first_equation - 1).unwrap()) {
                line_type = CodeType::Variable;
            };
        }
    }

    let mut spaces_found: i32 = 0;
    for letter in line.chars() {
        if letter == ' ' { spaces_found += 1; } else { break }
    }
    return ShallowParsedLine {
        line_code_type: line_type.clone(),
        actual_line: line.to_string(),
        all_spaces: spaces_found,
        placement: Some(i as i32),
    };
}
impl ShallowParsedLine {
    pub async fn from(python_code: String) -> Vec<ShallowParsedLine> {
        let mut threads:  Vec<JoinHandle<()>> = Vec::new();
        let result: Arc<RwLock<Vec<ShallowParsedLine>>> = Arc::new(RwLock::new(Vec::new()));

        for (i, line) in python_code.lines().enumerate().clone() {
            let i_owned = i.to_owned();
            let line_owned = line.to_owned();
            let result = result.clone();
            threads.push(spawn(async move{
                let shallow_line = from_parse(i_owned, line_owned).await;
                result.write().await.push(shallow_line);
            }));
        }
        for thread in threads{
            thread.await.unwrap();
        }
        let result = result.read().await;
        return result.clone();
    }

    pub fn empty(code: Option<String>) -> ShallowParsedLine {
        return ShallowParsedLine{
            line_code_type: CodeType::Unknown,
            actual_line: code.unwrap_or("".to_string()),
            all_spaces: 0,
            placement: None,
        };
    }
}


// a basic variable structure
#[derive(Debug, Clone)]
#[pyclass]
pub struct BaseVar {
    pub name: String,
    pub value: String,
    pub annotation: Option<String>,
    pub actual_line: String,
}

// rust only functions
#[allow(unused_assignments)]
impl BaseVar {
        pub fn from(shallow_var: ShallowParsedLine) -> PyResult<BaseVar> {
            if shallow_var.line_code_type.type_id() != CodeType::Variable.type_id() {
                return Err(NotVarError::from(
                    format!("expected a var, got {:?}", shallow_var.line_code_type)).to_pyerr()
                )
            }
            else if !shallow_var.actual_line.contains('=') {
                return Err(
                    NotVarError::from("the variable given does not have a '='").to_pyerr()
                )
            }

            let break_point: usize = shallow_var.actual_line.find("=").unwrap();

            let name: String = shallow_var.actual_line[..break_point].to_string();
            let value: String = shallow_var.actual_line[break_point+1..].to_string();

            let mut annotation: Option<String> = Some(String::new());
            if name.contains(":") {annotation = Some(name[name.find(":").unwrap() + 2..].to_string());}
            else {annotation = None;}

            return Ok(BaseVar {
                name: name,
                value: value,
                annotation: annotation,
                actual_line: shallow_var.actual_line,
            })
    }
}
// python and rust functions
#[pymethods]
impl BaseVar {
    pub fn name(&self) -> String {return self.name.clone();}
    pub fn value(&self) -> String {return self.value.clone();}
    pub fn annotation(&self) -> Option<String> {return self.annotation.clone();}
    pub fn actual_line(&self) -> String {return self.actual_line.clone()}
}

// the basic statement structure
#[derive(Debug, Clone)]
#[pyclass]
pub struct BaseStatement {
    pub statement_type: StatementType,
    pub statement_variables: Vec<String>,
    pub actual_line: ShallowParsedLine,
    pub is_async: bool,
}

#[pymethods]
impl BaseStatement {
    pub fn statement_type(&self) -> StatementType {self.statement_type.clone()}
    pub fn statement_variables(&self) -> Vec<String> {self.statement_variables.clone()}
    pub fn actual_line(&self) -> ShallowParsedLine {self.actual_line.clone()}
    pub fn is_async(&self) -> bool {self.is_async}
}

// rust only functions
impl BaseStatement {
    pub fn from(line: ShallowParsedLine) -> Result<BaseStatement, PyErr> {
        let line_words = line.actual_line.split_whitespace();

        let mut is_async: bool = false;
        let first_word = line_words.to_owned().next().unwrap().replace(":", "");
        let statement_type = if first_word == "async" {
            is_async = true;
            StatementType::from(line_words.to_owned().nth(1).unwrap())}
        else {
            StatementType::from(line_words
                .clone()
                .nth(0)
                .unwrap_or("<no word>")
                .replace(":", "")
                .as_str())
        };
        if statement_type.is_err() {return Err(statement_type.unwrap_err())}
        let statement_type = statement_type.unwrap();
        let mut statement_variables: Vec<String> = Vec::new();
        for (i, word) in line_words.enumerate() {
            if i == 0 {continue}
            if is_async && (i as i32) == 1 {continue}
            statement_variables.push(word.to_string());
        }
        return Ok(BaseStatement {
            statement_type: statement_type,
            statement_variables:  statement_variables,
            actual_line: line,
            is_async: is_async,
        })
    }
}
#[allow(dead_code)]
pub enum ExecutableType {
    Variable (String),
    Function (String),
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct BaseExecutable {
    actual_line: ShallowParsedLine,
    components: Vec<String>,
}

#[pymethods]
impl BaseExecutable {
    pub fn actual_line(&self) -> ShallowParsedLine {self.actual_line.clone()}
    pub fn components(&self) -> Vec<String> {self.components.clone()}
}

#[allow(unused_variables)]
impl BaseExecutable {
    pub fn from(line: ShallowParsedLine) -> PyResult<BaseExecutable> {
        let components = line.actual_line
            .split(".")
            .map(|component| component.to_string())
            .collect::<Vec<String>>();
        return Ok(BaseExecutable {
            actual_line: line,
            components: components,
        });
    }
}