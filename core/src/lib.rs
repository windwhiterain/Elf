pub mod backend;
pub mod common;
pub mod graph;
pub mod help;
pub mod python;
pub mod resource;
pub mod starter;
pub mod ui;
pub mod ui_debug;

use pyo3::prelude::*;
use std::{collections::HashMap, iter, path::PathBuf, sync::Arc, sync::Weak};
use ui_debug::UIDebug;

use common::*;
///The only context for an elf applycation
#[pyclass(unsendable)]
pub struct Context {
    ///resource context
    pub resource: resource::Context,
    ///python interpreter
    pub py_context: python::Context,
}

#[pymethods]
impl Context {
    #[new]
    pub fn new(py_context: python::Context) -> Context {
        Context {
            resource: resource::Context::new(),
            py_context,
        }
    }
}
#[pymodule]
#[pyo3(name = "elf_py")]
fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>()?;
    ui::gen_module(py, m)?;
    python::gen_module(py, m)?;
    Ok(())
}
