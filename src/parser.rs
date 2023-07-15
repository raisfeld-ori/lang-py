pub enum CodeType{
    Variable,
    Comment,
    Executable,
    NoEffect,
}

// every part of the code can be found here
struct ParsedLine{
    line_type: CodeType,
    line: String,
}

pub fn shallow_parse(text: String) -> String{
    return text;
}