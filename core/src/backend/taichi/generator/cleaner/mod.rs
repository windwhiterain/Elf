use std::path::PathBuf;

use pyo3::{types::PyDict, Py};

use crate::help::absolute_path::AbsolutePath;
use crate::{help::file, python};

pub fn clean(mut file_paths: Vec<PathBuf>, context: &python::Context) {
    for path in &mut file_paths {
        *path = path.abs_path();
    }
    context.run(
        &PathBuf::from("./src/backend/taichi/generator/cleaner/cleaner.py"),
        |py| {
            let globals = PyDict::new(py);
            globals.set_item("file_paths", file_paths).unwrap();
            Some(globals)
        },
    )
}
