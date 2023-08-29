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
use ui::{resource_tree::ResourceTree, schema_tree, UIInfor};
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
    pub fn new(py_context: python::Context, resource: resource::Context) -> Context {
        Context {
            resource: resource,
            py_context,
        }
    }
}
#[pymethods]
impl Context {
    #[new]
    pub fn py_new(py_env_path: PathBuf, plugin_search_path: PathBuf) -> Context {
        Self::new(
            python::Context::new(py_env_path),
            resource::Context::new(vec![plugin_search_path]),
        )
    }
    pub fn load_resource(&mut self) {
        self.resource.load()
    }
    pub fn resource_infor(&self) -> ResourceTree {
        self.resource.gen_infor()
    }
    pub fn schema_infor(&self, id: usize) -> schema_tree::Node {
        self.resource
            .plugins_content
            .schemas
            .get(id)
            .unwrap()
            .gen_infor()
    }
}
#[pymodule]
#[pyo3(name = "elf_rust")]
fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>().unwrap();
    Ok(())
}
