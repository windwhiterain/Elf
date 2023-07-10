use elf::Context;

fn test() {
    let mut context = Context::new();
    context.init();
    let st = context.resources.schemas["light"].value.structure.clone();
    let node = elf::ui::schema_tree::get_current_node(&context);
}
