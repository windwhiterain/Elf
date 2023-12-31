use crate::common::Schema;
use crate::help::absolute_path::AbsolutePath;
use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::*;
use std::collections::HashMap;
use std::path::Path;
use std::{fs::*, io::Read, path::PathBuf};
#[derive(Clone, Debug)]
pub struct Context {
    pub env_path: PathBuf,
}
impl Context {
    pub fn new(env_path: PathBuf) -> Context {
        Context { env_path }
    }
}
impl Context {
    pub fn site_package_path(&self) -> PathBuf {
        self.env_path.join(PathBuf::from("Lib/site-packages"))
    }
    pub fn run(&self, path: impl AsRef<Path>, globals: impl FnOnce(Python) -> Option<&PyDict>) {
        let mut code = String::new();
        File::open(&path)
            .expect({
                let path_str = path.as_ref().to_str().unwrap();
                format!("python file path err!path:{path_str}").as_str()
            })
            .read_to_string(&mut code)
            .unwrap();
        Python::with_gil(|py| {
            let sys = py.import("sys").unwrap();
            let sys_path = sys.getattr("path").unwrap().downcast::<PyList>().unwrap();
            sys_path.insert(0, self.site_package_path()).unwrap();
            sys_path
                .insert(0, PathBuf::from("../core_py").abs_path())
                .unwrap();
            match py.run(&code, globals(py), None) {
                Ok(()) => (),
                Err(e) => {
                    let trace = e.traceback(py).unwrap();
                    let infor = trace.format().unwrap();
                    println!("python err:{e}\n{infor}");
                    panic!();
                }
            }
        });
    }
}
