use std::path::PathBuf;

use main::*;
fn main() {
    let path = PathBuf::from("../../ui/__init__.py");
    let env_path = PathBuf::from("C:/SoftWare/PyCharm/Envs/Elf");
    let py_runer = python::Context::new(env_path);
    py_runer.run(&path)
}
