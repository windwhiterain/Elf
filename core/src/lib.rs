pub mod backend;
pub mod common;
pub mod graph;
pub mod help;
pub mod python;
pub mod resource;
pub mod starter;
pub mod ui;
pub mod ui_debug;

use once_cell::sync::{Lazy, OnceCell};
use pyo3::prelude::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    iter,
    path::PathBuf,
    sync::Arc,
    sync::{Once, RwLock, Weak},
};
use ui_debug::UIDebug;

use common::*;
///The only context for an elf applycation
#[pyclass]
#[derive(Debug)]
pub struct Context {
    ///resource context
    pub resource: resource::Context,
    ///python interpreter
    pub py_context: python::Context,
}

impl Context {
    pub fn new(py_context: python::Context) -> Context {
        Context {
            resource: resource::Context::new(),
            py_context,
        }
    }
}
#[pymethods]
impl Context {
    #[new]
    pub fn py_new(path: PathBuf) -> Context {
        Self::new(python::Context::new(path))
    }
    pub fn load_plugins(&mut self) {
        self.resource.load_plugins()
    }
}
#[pymodule]
#[pyo3(name = "elf_rust")]
fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>().unwrap();
    Ok(())
}
