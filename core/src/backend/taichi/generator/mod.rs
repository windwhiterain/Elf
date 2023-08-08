pub mod code_line;
pub mod struct_def;
use std::{
    fs, io,
    ops::Deref,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

use crate::{
    common::{operator::data_operator::DataOperatorR, schema::SchemaR},
    graph::{self, Graph},
    help::{ecs::Entity, ConstPtr},
    resource::{
        container::Directory,
        plugin::{self, ROOT_PLUGIN},
        PluginsContent, Resources,
    },
};
use io::Write;

use self::{
    code_line::CodeLines,
    struct_def::{flaten_schema_name, schema_to_struct_code},
};

use super::{type_to_code, DATA_TYPE_MAP};

pub struct Generator<'a> {
    graph: &'a Graph,
    path: PathBuf,
}
impl<'a> crate::backend::Generator<'a> for Generator<'a> {
    fn generate(&self, context: &PluginsContent) {
        self.generate_package();
        self.generate_refered_files();
        self.generate_struct(&context.schemas);
        self.generate_data();
        self.generate_operator(&context.data_operators)
    }
}
impl<'a> Generator<'a> {
    pub fn new(graph: &'a Graph, path: PathBuf) -> Self {
        Generator { graph, path }
    }
    fn generate_package(&self) {
        fs::create_dir(&self.path).unwrap();
        let mut structs_file = fs::File::create(self.path.join("./__init__.py")).unwrap();
        let mut code = CodeLines::new();
    }
    fn generate_refered_files(&self) {
        for plugin in &self.graph.plugins {
            if plugin.as_ref().get_const_ptr() == ROOT_PLUGIN.as_ref().get_const_ptr() {
                continue;
            }
            let path = self
                .path
                .join(PathBuf::from("./".to_string() + &plugin.std.name));
            fs::create_dir(&path).unwrap();
            let code_path = plugin.get_code_file_node().path().clone();
            fs_extra::dir::copy(code_path, path, &CopyOptions::new().content_only(true)).unwrap();
        }
        let mut reader = fs::File::open("./src/backend/taichi/generator/common_file.py").unwrap();
        let mut writer = fs::File::create(self.path.join("./common.py")).unwrap();
        io::copy(&mut reader, &mut writer).unwrap();
    }
    fn generate_struct(&self, context: &Resources<SchemaR>) {
        let mut structs_file = fs::File::create(self.path.join("./structs.py")).unwrap();
        let mut code = CodeLines::new();
        //import
        code.write(0, "import taichi".to_string());
        //structs
        for schema in context.filter_by_plugins(&self.graph.plugins) {
            code.append(schema_to_struct_code(schema));
        }
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
    fn generate_data(&self) {
        let mut structs_file = fs::File::create(self.path.join("./datas.py")).unwrap();
        let mut code = CodeLines::new();
        //import
        code.write(0, "import structs".to_string());
        //define context
        code.write(0, "class Context:".to_string());
        code.write(1, "def __init__(self):".to_string());
        //data field
        for (id, data) in self.graph.datas.iter().enumerate() {
            let type_name = type_to_code(data);
            let data_name = flaten_data_name(id);
            code.write(2, format!("self.{data_name}:{type_name}=None"));
        }
        //interface field
        for (id, interface) in self.graph.interfaces.iter().enumerate() {
            let type_name = flaten_schema_name(interface.schema.upgrade().unwrap().as_ref());
            let interface_name = flaten_interface_name(id);
            code.write(2, format!("self.{interface_name}:structs.{type_name}=None"));
        }
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
    fn generate_operator(&self, context: &Resources<DataOperatorR>) {
        let mut structs_file = fs::File::create(self.path.join("./operators.py")).unwrap();
        let mut code = CodeLines::new();
        code.write(0, "import common".to_string());
        //data_oeprator
        for operator in context.filter_by_plugins(&self.graph.plugins) {
            let file_path = PathBuf::from(
                "./".to_string() + operator.std.plugin.upgrade().unwrap().std.name.as_str(),
            )
            .join(operator.local_path());
            let path_code = file_path.to_str().unwrap();
            code.write(
                0,
                format!("module=common.import_module_by_path(\"{path_code}\")"),
            );
            let flatened_operator_name = flaten_data_operator_name(
                operator.std.id.load(std::sync::atomic::Ordering::Relaxed),
            );
            let operator_name = &operator.std.name;
            code.write(
                0,
                format!("{flatened_operator_name}=module.{operator_name}"),
            )
        }
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
}
fn flaten_data_name(id: usize) -> String {
    format!("__elf_data_{id}")
}
fn flaten_interface_name(id: usize) -> String {
    format!("__elf_interface_{id}")
}
fn flaten_data_operator_name(id: usize) -> String {
    format!("__elf_data_operator_{id}")
}
#[test]
fn test() {
    let mut context = crate::starter::test_initialize();
    context.resource.load_plugins();
    let graph = graph::test_instance(&context.resource);
    let generator = Generator::new(&graph, "../generated/taichi1".into());
    crate::backend::Generator::generate(&generator, &context.resource.plugins_content);
}
