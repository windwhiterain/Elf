use crate::*;

#[test]
///Where to start elf
fn start() {
    let py_context = python::Context::new(PathBuf::from("C:/SoftWare/PyCharm/Envs/Elf"));
    let mut context = Context::new(py_context);
    context.init();
}