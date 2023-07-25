use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use crate::base_parser::*;
use crate::errors::*;
use crate::outputs::*;

mod base_parser;
mod errors;
mod outputs;

#[pyfunction]
fn initial_parse(text: String) -> PyResult<BaseOutput> {
    /*
    the initial parse takes in the raw python code, does a shallow parse,
    returns any error it finds. if no error were found, it parses the output of the shallow parse
    and returns the different components of the code.
     */
    let shallow_code = ShallowParsedLine::from_pycode(text);
    return Ok(BaseOutput::from(shallow_code));
}

#[pymodule]
fn parse(_py: Python, module: &PyModule) -> PyResult<()> {
    /*
    the parse module does just what the name says,
    it parses the python code given into different variables,
    and returns a parsed code
     */
    module.add_wrapped(wrap_pyfunction!(initial_parse))?;
    Ok(())
}

// the header file for all rust code.
#[pymodule]
fn rust_header(_py: Python, module: &PyModule) -> PyResult<()> {
    /*
    the header module. all other modules and functions are 'Pymodules' or 'Pyfunctions'
    that belong to this part
     */
    module.add_wrapped(wrap_pymodule!(parse))?;
    Ok(())
}
