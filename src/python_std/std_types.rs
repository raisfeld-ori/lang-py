use pyo3::prelude::*;
use crate::extras::outputs::BaseModule;
use crate::parsing::base_parser::BaseVar;
use crate::parsing::base_types::*;

static STRINGS: [char; 2] = ['\"', '\''];
static NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
static NAMED: [(&str, StdTypes); 11] = [("True", StdTypes::Bool), ("False", StdTypes::Bool),
    ("Set", StdTypes::Set), ("frozenset", StdTypes::FrozenSet),
    ("bytes", StdTypes::Bytes), ("int", StdTypes::Int), ("str", StdTypes::Str),
    ("float", StdTypes::Float), ("list", StdTypes::List), ("len", StdTypes::Int),
    ("bool", StdTypes::Bool)
];


#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub struct PythonType (pub Type);

#[allow(dead_code)]
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Type {
    Standard (StdTypes), Object (BaseObject),
    Method (BaseMethod), Module (BaseModule),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub enum StdTypes{
    Int, Float, Str, List, Dict, Set,
    Bool, Range, Bytes, ByteArray, Module,
    FrozenSet,
}

impl StdTypes {
    pub fn parse_value(var: BaseVar) -> Option<StdTypes> {
        let mut current_type: Option<StdTypes> = None;
        let mut saved_letter: char = ' ';

        for letter in var.actual_line.actual_line.chars(){
            if STRINGS.contains(&letter) {
                if saved_letter == letter {current_type = Some(StdTypes::Str);break}
                else {saved_letter = letter;}
            }
            else if NUMBERS.contains(&letter) { current_type = Some(StdTypes::Int); }
            else if letter == '.' && NUMBERS.contains(&saved_letter) {
                current_type = Some(StdTypes::Float);
                break
            }
        }
        for (name, name_type) in NAMED.iter() {
            if var.actual_line.actual_line.replace(" ", "").starts_with(name) {current_type = Some(name_type.clone())}
        }


        return current_type;
    }
}