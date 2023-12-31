mod extras;
mod parsing;
mod python_std;
mod building;


use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::wrap_pyfunction;
use parsing::base_parser::*;
use extras::outputs::*;
use parsing::object_parsing::*;
use extras::actions::*;
use python_std::std_types::*;
use parsing::type_parsing::*;
use building::structure::*;
use building::builder::*;

// the core parsing classes and functions.
#[pymodule]
fn parse(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(parse_methods))?;
    module.add_wrapped(wrap_pyfunction!(parse_objects))?;
    module.add_wrapped(wrap_pyfunction!(create_base_output))?;
    module.add_wrapped(wrap_pyfunction!(get_module))?;
    module.add_class::<StdTypes>()?;
    module.add_class::<BaseMethod>()?;
    module.add_class::<ShallowParsedLine>()?;
    module.add_class::<BaseObject>()?;
    module.add_class::<BaseVar>()?;
    module.add_class::<BaseStatement>()?;
    module.add_class::<BaseExecutable>()?;
    module.add_class::<Operations>()?;
    module.add_class::<AbstractSyntaxTree>()?;
    Ok(())
}

// a module for async (with no GIL) functions, and for simplifying the usage of this package
#[pymodule]
fn actions(_py: Python, module: &PyModule) -> PyResult<()>{
    module.add_wrapped(wrap_pyfunction!(async_scan))?;
    module.add_wrapped(wrap_pyfunction!(async_parse_methods))?;
    module.add_wrapped(wrap_pyfunction!(async_parse_objects))?;
    module.add_wrapped(wrap_pyfunction!(async_get_module))?;
    module.add_class::<PythonType>()?;
    module.add_class::<AllOutputs>()?;
    module.add_class::<BaseCode>()?;
    module.add_class::<BaseModule>()?;
    Ok(())
}

#[pymodule]
fn build(_py: Python, module: &PyModule) -> PyResult<()>{
    module.add_class::<Builder>()?;
    module.add_class::<Structure>()?;
    module.add_class::<Format>()?;
    Ok(())
}

// the start of the code and the main module for this package
#[pymodule]
fn lang_py(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pymodule!(parse))?;
    module.add_wrapped(wrap_pymodule!(actions))?;
    module.add_wrapped(wrap_pymodule!(build))?;
    Ok(())
}
