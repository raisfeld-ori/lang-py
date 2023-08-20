use std::sync::Arc;
use pyo3::prelude::*;
use crate::parsing::base_parser::*;
use crate::extras::outputs::*;
use tokio::runtime::Builder;
use tokio::spawn;
use std::thread;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use crate::python_std::std_types::Type;
use crate::parsing::base_types::*;

// scans the raw python code and turns it into variables, statements, executables and unknown
#[pyfunction]
pub fn async_scan(text: String) -> PyResult<BaseCode> {
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
pub fn async_parse_methods(statements: Vec<BaseStatement>, all_lines: Vec<ShallowParsedLine>) -> PyResult<Vec<BaseMethod>> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let thread: thread::JoinHandle<PyResult<Vec<BaseMethod>>>= thread::spawn(move ||{
            runner.block_on(async move {
                let mut threads: Vec<JoinHandle<Option<PyResult<BaseMethod>>>> = Vec::new();
                for statement in statements{
                    let statement_owned = statement.to_owned();
                    let all_lines_owned = all_lines.to_owned();
                    threads.push(spawn(async move {
                        if statement_owned.statement_type == StatementType::Def{
                            Some(BaseMethod::from(statement_owned, all_lines_owned))
                        }
                        else {
                            None
                        }
                    }));
                }
                let mut base_methods: Vec<BaseMethod> = Vec::new();
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
// takes in ShallowParsedLine, which is a wrapper for the raw python code,
// and for every line it parses them further and sorts them as vars, statements, exes and others
pub async fn async_create_base_output(shallow_code: Vec<ShallowParsedLine>) -> PyResult<BaseCode>{
    let mut threads: Vec<JoinHandle<Option<PyErr>>> = Vec::new();
    let variables: Arc<RwLock<Vec<BaseVar>>> = Arc::new(RwLock::new(Vec::new()));
    let statements: Arc<RwLock<Vec<BaseStatement>>> = Arc::new(RwLock::new(Vec::new()));
    let executables: Arc<RwLock<Vec<BaseExecutable>>> = Arc::new(RwLock::new(Vec::new()));
    let unknowns: Arc<RwLock<Vec<Unknown>>> = Arc::new(RwLock::new(Vec::new()));
    let imports: Arc<RwLock<Vec<BaseStatement>>> = Arc::new(RwLock::new(Vec::new()));

    for shallow_line in shallow_code.iter() {
        let variables = variables.clone();
        let statements = statements.clone();
        let executables = executables.clone();
        let unknowns = unknowns.clone();
        let shallow_line = shallow_line.clone();
        let imports = imports.clone();
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
                    let statement = statement.unwrap();

                    if statement.clone().statement_type == StatementType::Import {
                        imports.write().await.push(statement.clone())
                    }

                    statements.write().await.push(statement);
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
    let imports = imports.read().await;

    return Ok(BaseCode {
        variables: variables.clone(),
        statements: statements.clone(),
        executables: executables.clone(),
        unknown: unknowns.clone(),
        shallow_code: shallow_code.clone(),
        imports: imports.clone(),
        })
}

// an async function that parses all statements and parses the objects
#[pyfunction]
pub fn async_parse_objects(statements: Vec<BaseStatement>, all_lines: Vec<ShallowParsedLine>, methods: Vec<BaseMethod>) -> PyResult<Vec<BaseObject>> {
    let runner = Builder::new_multi_thread().build().unwrap();
    let thread: thread::JoinHandle<PyResult<Vec<BaseObject>>>= thread::spawn(move ||{
            runner.block_on(async move {
                let mut threads: Vec<JoinHandle<Option<PyResult<BaseObject>>>> = Vec::new();
                for statement in statements{
                    let statement_owned = statement.to_owned();
                    let all_lines_owned = all_lines.to_owned();
                    let methods_owned = methods.to_owned();
                    threads.push(spawn(async move {
                        if statement_owned.statement_type == StatementType::Class{
                            Some(BaseObject::from(statement_owned, all_lines_owned, methods_owned))
                        }
                        else {
                            None
                        }
                    }));
                }
                let mut base_methods: Vec<BaseObject> = Vec::new();
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

// a combination of other async functions, in order to parse the code as fast as possible.
#[pyfunction]
pub fn async_get_module(text: String, name: String) -> PyResult<BaseModule> {
    let base_code = async_scan(text).unwrap();
    let methods = async_parse_methods(base_code.statements.clone(), base_code.shallow_code.clone());
    if methods.is_err() {return Err(methods.unwrap_err())}
    let methods: Vec<BaseMethod> = methods.unwrap();
    let objects = async_parse_objects(base_code.statements.clone(), base_code.shallow_code.clone(), methods.clone());
    if objects.is_err() {return Err(objects.unwrap_err())}
    let objects = objects.unwrap();
    let mut positions: Vec<usize> = Vec::new();
    let mut all_global: HashMap<String, Type> = HashMap::new();
    let mut global_objects: Vec<BaseObject> = Vec::new();
    let mut global_methods: Vec<BaseMethod> = Vec::new();
    for object in objects {
        if object.actual_line.actual_line.all_spaces == 0 {
            positions.push(object.actual_line.actual_line.position);
            for line in object.lines.iter() {positions.push(line.position)}
            global_objects.push(object.clone());
            all_global.insert(object.name.clone(),  Type::Object(object));
        }
    }
    for method in methods {
        if method.actual_line.actual_line.all_spaces == 0 {
            positions.push(method.actual_line.actual_line.position);
            for line in method.lines.iter() {positions.push(line.position)}
            global_methods.push(method.clone());
            all_global.insert(method.name.clone(), Type::Method(method.clone()));
        }
    }
    let variables: Vec<BaseVar> = base_code.variables
        .into_iter()
        .filter(|var|
            !positions.contains(&var.actual_line.position)
        && var.actual_line.all_spaces == 0)
        .collect();
    let statements: Vec<BaseStatement> = base_code.statements
        .into_iter()
        .filter(|statement|
            !positions.contains(&statement.actual_line.position)
        && statement.actual_line.all_spaces == 0)
        .collect();
    let executables: Vec<BaseExecutable> = base_code.executables
        .into_iter()
        .filter(|exe|
            !positions.contains(&exe.actual_line.position)
        && exe.actual_line.all_spaces == 0)
        .collect();
    let unknown: Vec<Unknown> = base_code.unknown
        .into_iter()
        .filter(|unknown|
            !positions.contains(&unknown.actual_line.position)
        && unknown.actual_line.all_spaces == 0)
        .collect();
    let shallow_code: Vec<ShallowParsedLine> = base_code.shallow_code
        .into_iter()
        .filter(|line|
            !positions.contains(&line.position)
        && line.all_spaces == 0
        )
        .collect();
    let global_code = BaseCode {
        statements: statements,
        variables: variables,
        executables: executables,
        unknown: unknown,
        shallow_code: shallow_code,
        imports: base_code.imports,
    };

    return Ok(BaseModule {
        name: name,
        methods: global_methods,
        objects: global_objects,
        code: global_code,
        all: all_global,
    })
}