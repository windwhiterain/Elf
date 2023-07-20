//!All implmentation for some important elf struct to display in isolated ui window

use crate::{hashmap, ui::schema_tree};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Weak},
};

use pyo3::{prelude::*, types::PyDict};

use crate::{
    common::Schema,
    resource::Resource,
    ui::{self, schema_tree::Node},
};
use ui::*;
///Display an elf struct in it's ui in a popup window.
pub trait UIDebug {
    ///Display an elf struct in it's ui in a popup window.
    fn display(&self, context: &crate::Context);
}
impl UIDebug for Resource<Schema> {
    fn display(&self, context: &crate::Context) {
        context
            .py_context
            .run(&PathBuf::from("./src/ui_debug/window.py"), |py| {
                let globals = PyDict::new(py);
                globals
                    .set_item("infor", Py::new(py, self.gen_infor()).unwrap())
                    .unwrap();
                Some(globals)
            });
    }
}
