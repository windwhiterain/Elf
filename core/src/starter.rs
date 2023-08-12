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
    let mut context = Context::new(py_context);
    context.resource.load_plugins();
    context.resource.plugins_content.schemas.find(
        &vec!["test_plugin1".to_string(), "complex".to_string()],
        None,
    );
}
