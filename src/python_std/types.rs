use pyo3::prelude::*;

static STRINGS: [char; 2] = ['\"', '\''];
static NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

#[derive(Clone, Debug, PartialOrd, PartialEq)]
#[pyclass]
pub enum StdTypes{
    Int, Float, Str, List, Dict, Set,
    Bool, Range, Bytes, ByteArray, Module,
    FrozenSet, ContextManager,
}

#[pymethods]
impl StdTypes {
    #[staticmethod]
    pub fn parse_from(val: String) -> Option<StdTypes> {
        let mut current_type: Option<StdTypes> = None;
        let mut saved_letter: char = ' ';

        for letter in val.chars(){
            if STRINGS.contains(&letter) {
                if saved_letter == letter {current_type = Some(StdTypes::Str);break}
                else {saved_letter = letter;}
            }
            else if NUMBERS.contains(&letter) { current_type = Some(StdTypes::Int); }
            else if letter == '[' {saved_letter = '[';}
            else if letter == ']' && saved_letter == '[' {current_type = Some(StdTypes::List);break}
        }

        return current_type;
    }
}