use std::{
    fs, io,
    ops::Deref,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

use crate::{
    common::schema::SchemaR,
    graph::{self, Graph},
    help::ecs::Entity,
    resource::{plugin, Resources},
};

pub struct Generator<'a> {
    graph: &'a Graph,
    path: PathBuf,
}
impl<'a> crate::backend::Generator<'a> for Generator<'a> {
    fn generate(&self) {
        self.generate_folder();
        self.generate_refered_files();
    }
}
impl<'a> Generator<'a> {
    pub fn new(graph: &'a Graph, path: PathBuf) -> Self {
        Generator { graph, path }
    }
    fn collect_input(&mut self, graph: &'a Graph, path: PathBuf) {
        self.graph = graph;
        self.path = path;
    }
    fn generate_folder(&self) {
        fs::create_dir(&self.path).unwrap();
    }
    fn generate_refered_files(&self) {
        for _plugin in &self.graph.plugins {
            let plugin = _plugin.upgrade().unwrap();
            let path = self
                .path
                .join(PathBuf::from("./".to_string() + &plugin.std.name));
            fs::create_dir(&path).unwrap();
            let code_path = plugin.get_code_file_node().path().clone();
            fs_extra::dir::copy(code_path, path, &CopyOptions::new().content_only(true)).unwrap();
        }
        let mut reader = fs::File::open("./src/backend/taichi/generator/common_file.py").unwrap();
        let mut writer = fs::File::create(self.path.join("./common_file.py")).unwrap();
        io::copy(&mut reader, &mut writer).unwrap();
    }
    fn generate_struct(&self, schemas: Resources<SchemaR>) {
        let mut active_plugins = vec![plugin::ROOT_PLUGIN.deref().as_ref()];
        let addition_plugins =
            Vec::from_iter(self.graph.plugins.iter().map(|a| a.upgrade().unwrap()));
        active_plugins.append(&mut Vec::from_iter(
            addition_plugins.iter().map(|a| a.as_ref()),
        ));
        for schema in schemas.filter_by_plugins(&active_plugins) {}
    }
}
#[test]
fn test() {
    let mut context = crate::starter::test_initialize();
    context.resource.load_plugins();
    let graph = graph::test_instance(&context.resource);
    let generator = Generator::new(&graph, "../generated/taichi1".into());
    crate::backend::Generator::generate(&generator)
}
