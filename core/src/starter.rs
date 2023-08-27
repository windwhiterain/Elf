use crate::*;
pub fn test_initialize() -> Context {
    Context::py_new(
        PathBuf::from("C:/SoftWare/PyCharm/Envs/Elf"),
        PathBuf::from("../plugin"),
    )
}
///Where to start elf
#[test]
pub fn start() {
    let mut context = test_initialize();
    context.resource.load();
    let schema = context
        .resource
        .plugins_content
        .schemas
        .find(
            &vec!["test_plugin2".to_string(), "Complex".to_string()],
            None,
        )
        .unwrap();
    schema.show_graph(&context);
}
