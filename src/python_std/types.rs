use pyo3::prelude::*;

#[pyclass]
pub enum BasicType{
    Int, Float, Str, List, Dict, Set,
    Bool, Range, Bytes, ByteArray, Module,
    FrozenSet, ContextManager, Other,
}

impl BasicType {
    pub fn from_annotation(annotation: String) {

    }
}
