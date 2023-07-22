/*
parser:
the parser file does what the name says. it takes in different
arguments, parses them, and returns the output
 */
use std::collections::{hash_map, HashMap};
use std::any::{Any, TypeId};
use pyo3::prelude::*;
use pyo3::exceptions::PyAttributeError;
use crate::errors;

static OPERATORS: [char; 3] = ['>', '<', '!'];
static STATEMENTS: [&str; 13] = ["if", "else", "elif",
    "for", "def", "async", "try", "except",
    "finally", "break", "continue", "while",
    "class"];

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
    Variable = 0,
    Executable = 1,
    Unknown = 2,
    Statement = 3,
}

// the most basic parsing of the code
#[derive(Debug, Clone)]
#[allow(dead_code)]
#[pyclass]
pub struct ShallowParsedLine{
    pub line_code_type: CodeType,
    pub actual_line: String,
    pub all_spaces: i32,
    pub  statements_before: Vec<ShallowParsedLine>,
}

// creating new ShallowParsedLines based on the code given
// btw i can't use the From trait because it forces the from function to return ShallowParsedLine
impl ShallowParsedLine {
    pub fn from_pycode(python_code: String) -> Vec<ShallowParsedLine> {
        let mut result: Vec<ShallowParsedLine> = Vec::new();
        let mut statements_before: Vec<ShallowParsedLine> = Vec::new();

        for line in python_code.lines() {
            let mut line_type: CodeType = CodeType::Unknown;

            if STATEMENTS.contains(&line) {
                line_type = CodeType::Statement;
            }
            if line.ends_with(")") && line_type.type_id() == CodeType::Unknown.type_id() { line_type = CodeType::Executable }

            if line.contains("=") {
                let first_equation: usize = line.find("=").unwrap();
                if line.chars().nth(first_equation + 1 as usize).unwrap() != '=' {
                    if !OPERATORS.contains(&line.chars().nth(first_equation - 1 as usize).unwrap()) {
                        line_type = CodeType::Variable;
                    };
                }
            }

            let mut spaces_found: i32 = 0;
            for letter in line.chars() {
                if letter == ' ' { spaces_found += 1; } else { break }
            }

            result.push(ShallowParsedLine {
            line_code_type: line_type.clone(),
            actual_line: line.to_string(),
            all_spaces: spaces_found,
            statements_before: statements_before.clone(),
            });

            if line_type.type_id() == CodeType::Statement.type_id(){
                statements_before.push(ShallowParsedLine {
                    line_code_type: line_type.clone(),
                    actual_line: line.to_string(),
                    all_spaces: spaces_found,
                    statements_before: statements_before.clone(),
                });
            }
        }

        return result;
    }

    pub fn empty(code: Option<String>) -> ShallowParsedLine {
        return ShallowParsedLine{
                line_code_type: CodeType::Unknown,
                actual_line: code.unwrap_or("".to_string()),
                all_spaces: 0,
                statements_before: vec![],
                };
    }
}


// a basic variable structure
#[allow(dead_code, unused_variables)]
#[pyclass]
pub struct BaseVar {
    name: String,
    value: String,
    annotation: Option<String>,
    owner: ShallowParsedLine,
}

// implementation of the From trait<ShallowParsedLine>, not much else.
impl BaseVar {
        pub fn from(shallow_var: ShallowParsedLine) -> Result<BaseVar, PyErr> {
            if shallow_var.line_code_type.type_id() != CodeType::Variable.type_id() {
                return Err(PyAttributeError::new_err(
                format!("expected Variable, got {:?}", shallow_var.line_code_type.type_id()))
                ); }
            else if !shallow_var.actual_line.contains('=') {
                return Err(PyAttributeError::new_err(
                format!("invalid variable, missing '='")
                )); }

            let break_point: usize = shallow_var.actual_line.find("=").unwrap();

            let name: String = shallow_var.actual_line[..break_point].to_string();
            let value: String = shallow_var.actual_line[break_point+1..].to_string();

            let mut annotation: Option<String> = Some(String::new());
            if name.contains(":") {annotation = Some(name[name.find(":").unwrap()..].to_string());}
            else {annotation = None;}

            let mut owner: ShallowParsedLine = ShallowParsedLine::empty(Some("global".to_string()));

            for statement in shallow_var.statements_before{
                if statement.all_spaces >= shallow_var.all_spaces {continue;}

                if statement.actual_line.contains("class") || statement.actual_line.contains("def"){
                    owner = statement;
                    break;
                }
            }
            return Ok(BaseVar {
                name: name,
                value: value,
                annotation: annotation,
                owner: owner,
            })
    }
}

// functions for debugging the output
#[pymethods]
impl BaseVar {
    pub fn name(&self) -> String {self.name.clone()}
    pub fn value(&self) -> String {self.value.clone()}
    pub fn annotation(&self) -> Option<String> {self.annotation.clone()}
    pub fn owner(&self) -> ShallowParsedLine {self.owner.clone()}
}