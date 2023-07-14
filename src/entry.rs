pub enum CodeType{
    Variable,
    Comment,
    Executable,
}

// every part of the code can be found here
struct ParsedLine{
    line_type: CodeType,
    line: String,
}

pub fn parse_text(text: String){println!("text"); }