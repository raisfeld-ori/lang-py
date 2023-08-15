mod extras;
mod parsing;


use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::wrap_pyfunction;
use parsing::base_parser::*;
use parsing::outputs::*;
use parsing::base_types::*;
use extras::actions::*;

// the core parsing classes and functions.
#[pymodule]
fn parse(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(parse_methods))?;
    module.add_wrapped(wrap_pyfunction!(parse_objects))?;
    module.add_wrapped(wrap_pyfunction!(create_base_output))?;
    module.add_class::<Method>()?;
    module.add_class::<ShallowParsedLine>()?;
    module.add_class::<Object>()?;
    module.add_class::<BaseVar>()?;
    module.add_class::<BaseStatement>()?;
    module.add_class::<BaseExecutable>()?;
    Ok(())
}

// a module for async (with no GIL) functions, and for simplifying the usage of this package
#[pymodule]
fn actions(_py: Python, module: &PyModule) -> PyResult<()>{
    module.add_wrapped(wrap_pyfunction!(async_scan))?;
    module.add_wrapped(wrap_pyfunction!(async_parse_methods))?;
    module.add_wrapped(wrap_pyfunction!(async_parse_objects))?;
    module.add_class::<AllOutputs>()?;
    module.add_class::<BaseOutput>()?;
    Ok(())
}

// the start of the code and the main module for this package
#[pymodule]
fn lang_py(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pymodule!(parse))?;
    module.add_wrapped(wrap_pymodule!(actions))?;
    Ok(())
}
