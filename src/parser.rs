/*
parser:
the parser file does what the name says. it takes in different
arguments, parses them, and returns the output
 */
use std::collections::{hash_map, HashMap};
use std::any::Any;

static OPERATORS: [char; 3] = ['>', '<', '!'];

static STATEMENTS: [&str; 13] = ["if", "else", "elif",
    "for", "def", "async", "try", "except",
    "finally", "break", "continue", "while",
    "class"];
static IMPORTS: [&str; 2] = ["import", "from"];

#[derive(Debug)]
pub enum CodeType{
    Variable,
    Executable,
    Unknown,
    Statement,
    Import,
}


// every part of the code can be found here
#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedLine{
    line_type: CodeType,
    line: String,
    spaces: i32,
}
impl ParsedLine {
    pub fn line(self) -> String {self.line}
    pub fn spaces(self) -> i32 {self.spaces}
    pub fn line_type(self) -> CodeType {self.line_type}
}

pub fn shallow_parse(text: String) -> Vec<ParsedLine>{
    /*
    shallow parse takes in raw python code,
    and returns the basic info on that code, such
    as the line, the type etc.
     */
    let mut result: Vec<ParsedLine> = Vec::new();

    for line in text.lines(){
        let mut line_type: CodeType = CodeType::Unknown;

        for statement in STATEMENTS{ if line.starts_with(statement) {line_type = CodeType::Statement;} }
        if line.ends_with(")") && line_type.type_id() == CodeType::Unknown.type_id() {line_type = CodeType::Executable}
        if line.contains("=") {
            let first_equation: usize = line.find("=").unwrap();
            if line.chars().nth(first_equation + 1 as usize).unwrap() != '='{
                if !OPERATORS.contains(&line.chars().nth(first_equation - 1 as usize).unwrap()) {line_type = CodeType::Variable;};
            }
        }


        let mut spaces: i32 = 0;
        for char in line.chars(){
            if char != ' ' {break}
            spaces += 1;
        }

        result.push(ParsedLine {
            line_type: line_type,
            spaces: spaces,
            line: line.to_string(),
        });
    }

    return result;
}

pub fn component_parse(shallow_code: Vec<ParsedLine>) {
    let imports: Vec<String> = Vec::new();
    let variables: HashMap<String, String> = HashMap::new();


    for shallow_line in shallow_code {

        match shallow_line.line_type{
            CodeType::Variable => {

            }
            _ => {}
        }
    }
}