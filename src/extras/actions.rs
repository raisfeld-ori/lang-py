use std::sync::Arc;
use pyo3::prelude::*;
use crate::parsing::base_parser::*;
use crate::parsing::outputs::*;
use crate::parsing::base_parser::*;
use tokio::runtime::Builder;
use tokio::spawn;
use std::thread;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::parsing::base_types::*;

/* scans the basic code structure asynchronously without python's GIL
:param text - the raw python code
:return - returns PyResult<BaseOutput>, which is Result<crate::parsing::outputs::BaseOutput, PyErr>
 */
#[pyfunction]
pub fn async_scan(text: String) -> PyResult<BaseOutput> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let output = thread::spawn(move ||{
        runner.block_on(async move {
            let shallow_code  = ShallowParsedLine::async_from(text).await;
            async_create_base_output(shallow_code).await
        })
    });
    return output.join().unwrap();
}


#[pyfunction]
pub fn async_parse_methods(statements: Vec<BaseStatement>, all_lines: Vec<ShallowParsedLine>) -> PyResult<Vec<Method>> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let thread: thread::JoinHandle<PyResult<Vec<Method>>>= thread::spawn(move ||{
            runner.block_on(async move {
                let mut threads: Vec<JoinHandle<Option<PyResult<Method>>>> = Vec::new();
                for statement in statements{
                    let statement_owned = statement.to_owned();
                    let all_lines_owned = all_lines.to_owned();
                    threads.push(spawn(async move {
                        if statement_owned.statement_type == StatementType::Def{
                            Some(Method::from(statement_owned, all_lines_owned))
                        }
                        else {
                            None
                        }
                    }));
                }
                let mut base_methods: Vec<Method> = Vec::new();
                for thread in threads{
                    let output = thread.await.unwrap();
                    if output.is_none() {continue;}
                    let output = output.unwrap();
                    if output.is_err() {return Err(output.unwrap_err());}
                    else {base_methods.push(output.unwrap());}
                }
                return Ok(base_methods);

            })
        });
    return thread.join().unwrap();
}

pub async fn async_create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseOutput>{
    let mut threads: Vec<JoinHandle<Option<PyErr>>> = Vec::new();
    let variables: Arc<RwLock<Vec<BaseVar>>> = Arc::new(RwLock::new(Vec::new()));
    let statements: Arc<RwLock<Vec<BaseStatement>>> = Arc::new(RwLock::new(Vec::new()));
    let executables: Arc<RwLock<Vec<BaseExecutable>>> = Arc::new(RwLock::new(Vec::new()));
    let unknowns: Arc<RwLock<Vec<Unknown>>> = Arc::new(RwLock::new(Vec::new()));

    for shallow_line in shallow_code.iter() {
        let variables = variables.clone();
        let statements = statements.clone();
        let executables = executables.clone();
        let unknowns = unknowns.clone();
        let shallow_line = shallow_line.clone();
        threads.push(spawn(async move{
            match shallow_line.line_code_type {
                CodeType::Variable => {
                    let variable = BaseVar::from(shallow_line.to_owned());
                    if variable.is_err() {return Some(variable.unwrap_err())}
                    variables.write().await.push(variable.unwrap());
                }
                CodeType::Statement => {
                    let statement = BaseStatement::from(shallow_line.to_owned());
                    if statement.is_err() {return Some(statement.unwrap_err())}
                    statements.write().await.push(statement.unwrap());
                }
                CodeType::Executable => {
                    let executable = BaseExecutable::from(shallow_line.to_owned());
                    if executable.is_err() {return Some(executable.unwrap_err())}
                    executables.write().await.push(executable.unwrap());
                }
                CodeType::Unknown => {
                    let unknown = Unknown::from(shallow_line.to_owned());
                    if unknown.is_err() {return Some(unknown.unwrap_err())}
                    unknowns.write().await.push(unknown.unwrap());
                }
            };
            return None;
        }));
    }

    for thread in threads {
        let error = thread.await.unwrap();
        if error.is_some() {return Err(error.unwrap())}
    }


    let variables = variables.read().await;
    let statements = statements.read().await;
    let executables = executables.read().await;
    let unknowns = unknowns.read().await;

    return Ok(BaseOutput {
        variables: variables.clone(),
        statements: statements.clone(),
        executables: executables.clone(),
        unknown: unknowns.clone(),
        shallow_code: shallow_code.clone(),
        })
}

#[pyfunction]
pub fn async_parse_objects(statements: Vec<BaseStatement>, all_lines: Vec<ShallowParsedLine>, methods: Vec<Method>) -> PyResult<Vec<Object>> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let thread: thread::JoinHandle<PyResult<Vec<Object>>>= thread::spawn(move ||{
            runner.block_on(async move {
                let mut threads: Vec<JoinHandle<Option<PyResult<Object>>>> = Vec::new();
                for statement in statements{
                    let statement_owned = statement.to_owned();
                    let all_lines_owned = all_lines.to_owned();
                    let methods_owned = methods.to_owned();
                    threads.push(spawn(async move {
                        if statement_owned.statement_type == StatementType::Class{
                            Some(Object::from(statement_owned, all_lines_owned, methods_owned))
                        }
                        else {
                            None
                        }
                    }));
                }
                let mut base_methods: Vec<Object> = Vec::new();
                for thread in threads{
                    let output = thread.await.unwrap();
                    if output.is_none() {continue;}
                    let output = output.unwrap();
                    if output.is_err() {return Err(output.unwrap_err());}
                    else {base_methods.push(output.unwrap());}
                }
                return Ok(base_methods);

            })
        });
    return thread.join().unwrap();
}