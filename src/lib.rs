mod base_parser;
mod errors;
mod outputs;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use tokio::runtime::Builder;
use std::thread;
use crate::base_parser::*;
use crate::outputs::*;

#[pyfunction]
// takes in raw python code, parses it into variables, statements, executables, and unknown.
fn initial_parse(text: String) -> PyResult<BaseOutput> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let output = thread::spawn(move ||{
        runner.block_on(async move {
            let shallow_code  = ShallowParsedLine::from(text).await;
            create_base_output(shallow_code).await
        })
    });
    return output.join().unwrap();
}


// all classes python calls explicitly
#[pymodule]
fn classes(_py: Python, module: &PyModule) -> PyResult<()> {

    module.add_class::<AllOutputs>()?;
    Ok(())
}

// the functions for parsing the python code
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
fn compiler(_py: Python, module: &PyModule) -> PyResult<()> {
    /*
    the header module. all other modules and functions are 'Pymodules' or 'Pyfunctions'
    that belong to this part
     */
    module.add_wrapped(wrap_pymodule!(parse))?;
    module.add_wrapped(wrap_pymodule!(classes))?;
    Ok(())
}
