use pyo3::PyResult;

pub trait Object{
    fn parse(value: String) -> PyResult<Self>;
    fn try_parse(value: String) -> Option<Self>;
}