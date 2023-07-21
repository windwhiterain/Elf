use crate::*;
pub fn test_initialize() -> Context {
    let py_context = python::Context::new(PathBuf::from("C:/SoftWare/PyCharm/Envs/Elf"));
    let context = Context::new(py_context);
    context
}
///Where to start elf
#[test]
pub fn start() {
    let py_context = python::Context::new(PathBuf::from("C:/SoftWare/PyCharm/Envs/Elf"));
    let context = Context::new(py_context);
}
