mod base_parser;
mod errors;
mod outputs;
mod base_types;


use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use tokio::runtime::Builder;
use std::thread;
use crate::base_parser::*;
use crate::outputs::*;
use crate::base_types::{get_base_methods, Method, Object, get_base_objects};

// takes in raw python code, parses it into variables, statements, executables, and unknown.
#[pyfunction]
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

// the functions for parsing the python code
#[pymodule]
fn parse(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(initial_parse))?;
    module.add_wrapped(wrap_pyfunction!(get_base_methods))?;
    module.add_wrapped(wrap_pyfunction!(get_base_objects))?;
    module.add_class::<Method>()?;
    module.add_class::<ShallowParsedLine>()?;
    module.add_class::<Object>()?;
    module.add_class::<BaseVar>()?;
    module.add_class::<BaseStatement>()?;
    module.add_class::<BaseExecutable>()?;
    module.add_class::<AllOutputs>()?;
    module.add_class::<BaseOutput>()?;
    Ok(())
}

// the header file for all rust code.
#[pymodule]
fn lang_py(_py: Python, module: &PyModule) -> PyResult<()> {
    /*
    the header module. all other modules and functions are 'Pymodules' or 'Pyfunctions'
    that belong to this part
     */
    module.add_wrapped(wrap_pymodule!(parse))?;
    Ok(())
}
