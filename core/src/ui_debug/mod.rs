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
pub fn show_schema(schema: &Arc<Resource<Schema>>, context: &mut crate::Context) {
    context
        .py_context
        .run(&PathBuf::from("./src/ui_debug/window.py"), |py| {
            let globals = PyDict::new(py);
            globals
                .set_item("infor", Py::new(py, schema_tree::get_node(schema)).unwrap())
                .unwrap();
            Some(globals)
        });
}
