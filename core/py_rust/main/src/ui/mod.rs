use crate::common::Schema;
use pyo3::prelude::*;
use std::{fs::*, io::Read, path::Path};

pub fn transform_schema() {
    let path = Path::new("../../ui/__init__.py");
    let mut code = String::new();
    match File::open(&path) {
        Ok(mut f) => {
            f.read_to_string(&mut code);
        }
        Err(e) => {
            print!("{}", e);
        }
    }
    Python::with_gil(|py| {
        let res = py.run(&code, None, None);
        //let ui = PyModule::from_code(py, &code, "__init__", "ui");
        match res {
            Ok(()) => {
                print!("yes");
            }
            Err(e) => {
                print!("{}", e);
            }
        }
    })
}
