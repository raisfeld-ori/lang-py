/*
base_parser:
does the basic parsing of the code, meaning it parses things like the code structure,
the name and value of a variable etc.
 */
use std::any::Any;
use std::ptr::write;
use pyo3::callback::IntoPyCallbackOutput;
use pyo3::prelude::*;
use crate::errors::*;


static OPERATORS: [char; 3] = ['>', '<', '!'];
static STATEMENTS: [&str; 12] = ["if", "else", "elif",
    "for", "def", "async", "try", "except",
    "finally", "while",
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
    Variable = 0,
    Executable = 1,
    Unknown = 2,
    Statement = 3,
}


// the most basic parsing of the code
#[derive(Debug, Clone)]
#[pyclass]
pub struct ShallowParsedLine{
    pub line_code_type: CodeType,
    pub actual_line: String,
    pub all_spaces: i32,
    pub statements_before: Vec<ShallowParsedLine>,
    pub placement: Option<i32>,
}

#[pymethods]
impl ShallowParsedLine {
    pub fn line_code_type(&self) -> CodeType {self.line_code_type.clone()}
    pub fn actual_line(&self) -> String {self.actual_line.clone()}
    pub fn all_spaces(&self) -> i32 {self.all_spaces.clone()}
    pub fn statements_before(&self) -> Vec<ShallowParsedLine> {self.statements_before.clone()}
    pub fn placement(&self) -> Option<i32> {self.placement.clone()}
}

// creating new ShallowParsedLines based on the code given
// btw i can't use the From trait because it forces the from function to return ShallowParsedLine
impl ShallowParsedLine {
    pub fn from_pycode(python_code: String) -> Vec<ShallowParsedLine> {
        let mut result: Vec<ShallowParsedLine> = Vec::new();
        let mut statements_before: Vec<ShallowParsedLine> = Vec::new();

        for (i, line) in python_code.lines().enumerate() {
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
                placement: Some(i as i32),
            });

            if line_type.type_id() == CodeType::Statement.type_id(){
                statements_before.push(ShallowParsedLine {
                    line_code_type: line_type.clone(),
                    actual_line: line.to_string(),
                    all_spaces: spaces_found,
                    statements_before: statements_before.clone(),
                    placement: Some(i as i32),
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
    pub owner: Option<BaseStatement>,
}

// rust only functions
impl BaseVar {
        pub fn from(shallow_var: ShallowParsedLine) -> Result<BaseVar, NotVarError> {
            if shallow_var.line_code_type.type_id() != CodeType::Variable.type_id() {
                return Err(NotVarError (format!("expected a var, got {:?}", shallow_var.line_code_type)))
            }
            else if !shallow_var.actual_line.contains('=') {
                return Err(NotVarError ("the variable given does not have a '='".to_string()))
            }

            let break_point: usize = shallow_var.actual_line.find("=").unwrap();

            let name: String = shallow_var.actual_line[..break_point].to_string();
            let value: String = shallow_var.actual_line[break_point+1..].to_string();

            let mut annotation: Option<String> = Some(String::new());
            if name.contains(":") {annotation = Some(name[name.find(":").unwrap()..].to_string());}
            else {annotation = None;}

            let mut owner: Option<BaseStatement> = None;

            for statement in shallow_var.statements_before{
                if statement.all_spaces >= shallow_var.all_spaces {continue;}

                if statement.actual_line.contains("class") || statement.actual_line.contains("def"){
                    let new_owner: Result<BaseStatement, NotStatementError> = BaseStatement::from(statement);
                    match new_owner {
                        Err(statement_error) => {
                            return Err(NotVarError (format!("the owner returned an error:\n{}", statement_error)))
                        }
                        Ok(new_statement) => {
                            owner = Some(new_statement);
                        }
                    }


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
    pub fn new(name: String, value: String, annotation: Option<String>, owner: Option<BaseStatement>) -> BaseVar {
        return BaseVar {
            name: name,
            value: value,
            annotation: annotation,
            owner: owner,
        }
    }
}
// python and rust functions
#[pymethods]
impl BaseVar {
    pub fn name(&self) -> String {return self.name.clone();}
    pub fn value(&self) -> String {return self.value.clone();}
    pub fn annotation(&self) -> Option<String> {return self.annotation.clone();}
    pub fn owner(&self) -> Option<BaseStatement> {return self.owner.clone();}
}

// every type of statement
#[derive(Debug, Clone)]
#[pyclass]
pub enum StatementType {
    For, While, Import, Try, Except, If,
    Else, Elif, With, Class, Finally,
}

// rust only functions
impl StatementType{
    pub fn from(word: &str) -> Result<StatementType, NotStatementError> {
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
            _ => {Err(NotStatementError ("could not parse the statement type".to_string()))}
        }
    }
}

// the basic statement structure
#[derive(Debug, Clone)]
#[pyclass]
pub struct BaseStatement {
    pub statement_type: StatementType,
    pub statement_variables: Vec<String>,
    pub is_async: bool,
    pub owner: Option<Box<BaseStatement>>,
}

// rust only functions
impl BaseStatement {
    pub fn from(line: ShallowParsedLine) -> Result<BaseStatement, NotStatementError> {
        let line_words = line.actual_line.split_whitespace();

        let mut is_async: bool = false;
        let statement_type = if line_words.clone().next().unwrap() == "async" {
            is_async = true;
            StatementType::from(line_words.clone().nth(0).unwrap())}
        else {
            StatementType::from(line_words.clone().nth(1).unwrap())
        };
        if statement_type.is_err() {return Err(statement_type.unwrap_err())}
        let statement_type = statement_type.unwrap();
        let mut owner: Option<Box<BaseStatement>> = None;
        for statement in line.statements_before{
                if statement.all_spaces >= line.all_spaces {continue;}

                if statement.actual_line.contains("class") || statement.actual_line.contains("def"){
                    let new_owner: Result<BaseStatement, NotStatementError> = BaseStatement::from(statement);
                    match new_owner {
                        Err(error) => {
                            return Err(NotStatementError ("found an invalid owner".to_string()))
                        }
                        Ok(new_statement) => {
                            owner =Some(Box::new(new_statement));
                        }
                    }
                    break;
                }
            }
        let mut statement_variables: Vec<String> = Vec::new();
        for (i, word) in line_words.enumerate() {
            if i == 0 {continue}
            if is_async && (i as i32) == 1 {continue}
            statement_variables.push(word.to_string());
        }
        return Ok(BaseStatement {
            statement_type: statement_type,
            statement_variables:  statement_variables,
            is_async: is_async,
            owner: owner,
        })
    }
}