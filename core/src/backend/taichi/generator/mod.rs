pub mod cleaner;
pub mod code_line;
pub mod interface_operator;
pub mod struct_def;
use std::{
    fs, io,
    ops::Deref,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

use crate::{
    common::{operator::data_operator::DataOperatorR, schema::SchemaR},
    graph::{
        self,
        arbitary_data::ArbitaryData,
        node::{self, Node, StaticNodeType},
        operator, Graph,
    },
    help::{ecs::Entity, file, ConstPtr},
    python,
    resource::{
        container::Directory,
        plugin::{self, ROOT_PLUGIN},
        PluginsContent, Resources,
    },
    Context,
};
use io::Write;

use self::{
    code_line::CodeLines,
    struct_def::{flaten_schema_name, flaten_schema_ref_name, schema_to_struct_code},
};

use super::{type_to_code, DATA_TYPE_MAP};

pub struct Generator<'a> {
    graph: &'a Graph,
    path: PathBuf,
}
impl<'a> crate::backend::Generator<'a> for Generator<'a> {
    fn generate(&self, context: &Context) {
        self.generate_folder();
        self.generate_refered_files(&context.py_context);
        self.generate_struct(&context.resource.plugins_content.schemas);
        self.generate_context();
        self.generate_operator(&context.resource.plugins_content.data_operators);
        self.generate_node();
    }
}
impl<'a> Generator<'a> {
    pub fn new(graph: &'a Graph, path: PathBuf) -> Self {
        Generator { graph, path }
    }
    fn generate_folder(&self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).unwrap();
        }
        fs::create_dir(&self.path).unwrap();
    }
    fn generate_refered_files(&self, context: &python::Context) {
        let mut elf_script_paths = Vec::<PathBuf>::new();
        for plugin in &self.graph.plugins {
            if plugin.as_ref().get_const_ptr() == ROOT_PLUGIN.as_ref().get_const_ptr() {
                continue;
            }
            let to_path = self
                .path
                .join(PathBuf::from("./".to_string() + &plugin.std.name));
            fs::create_dir(&to_path).unwrap();
            let from_path = plugin.get_code_file_node().path().clone();
            fs_extra::dir::copy(from_path, &to_path, &CopyOptions::new().content_only(true))
                .unwrap();
            elf_script_paths.extend(
                file::Node::from(to_path.clone())
                    .get_files(".py")
                    .into_iter()
                    .map(|a| a.path().clone()),
            );
        }
        cleaner::clean(elf_script_paths, context);
        let mut reader = fs::File::open("./src/backend/taichi/generator/common.py").unwrap();
        let mut writer = fs::File::create(self.path.join("./common.py")).unwrap();
        io::copy(&mut reader, &mut writer).unwrap();
        let mut reader = fs::File::open("./src/backend/taichi/generator/help.py").unwrap();
        let mut writer = fs::File::create(self.path.join("./help.py")).unwrap();
        io::copy(&mut reader, &mut writer).unwrap();
        let mut reader = fs::File::open("./src/backend/taichi/generator/__init__.py").unwrap();
        let mut writer = fs::File::create(self.path.join("./__init__.py")).unwrap();
        io::copy(&mut reader, &mut writer).unwrap();
    }
    fn generate_struct(&self, context: &Resources<SchemaR>) {
        let mut structs_file = fs::File::create(self.path.join("./structs.py")).unwrap();
        let mut code = CodeLines::new();
        //import
        code.write(0, "import taichi".to_string());
        code.write(0, "from .help import Ref,ChainRef".to_string());
        code.write(0, "from .common import ShapeConstraint".to_string());
        //structs
        for schema in context.filter_by_plugins(&self.graph.plugins) {
            code.append(schema_to_struct_code(schema), 0);
        }
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
    fn generate_context(&self) {
        let mut structs_file = fs::File::create(self.path.join("./context.py")).unwrap();
        let mut code = CodeLines::new();
        //import
        code.write(0, "from . import structs".to_string());
        code.write(0, "from .help import Ref".to_string());
        //define context
        code.write(0, "class Context:".to_string());
        code.write(1, "def __init__(self):".to_string());
        //data field
        for (id, data) in self.graph.datas.iter().enumerate() {
            let data_name = flaten_data_name(id);
            code.write(2, format!("self.{data_name}=Ref(None)"));
        }
        //interface field
        for (id, interface) in self.graph.interfaces.iter().enumerate() {
            let type_name = flaten_schema_ref_name(interface.schema.upgrade().unwrap().as_ref());
            let interface_name = flaten_interface_name(id);
            code.write(
                2,
                format!("self.{interface_name}:structs.{type_name}=structs.{type_name}()"),
            );
        }
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
    fn generate_operator(&self, context: &Resources<DataOperatorR>) {
        let mut structs_file = fs::File::create(self.path.join("./operators.py")).unwrap();
        let mut code = CodeLines::new();
        //import
        code.write(0, "from .help import import_module_by_path".to_string());
        code.write(0, "from os import path".to_string());
        //data_oeprator
        for operator in context.filter_by_plugins(&self.graph.plugins) {
            let file_path = PathBuf::from(
                "/".to_string() + operator.std.plugin.upgrade().unwrap().std.name.as_str(),
            )
            .join(operator.local_path());
            let path_code = file_path.to_str().unwrap();
            code.write(0, format!("module=import_module_by_path(path.dirname(path.abspath(__file__))+\"{path_code}\")"));
            let flatened_operator_name = flaten_data_operator_name(
                operator.std.id.load(std::sync::atomic::Ordering::Relaxed),
            );
            let operator_name = &operator.std.name;
            code.write(
                0,
                format!("{flatened_operator_name}=module.{operator_name}()"),
            )
        }
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
    fn generate_node(&self) {
        let mut structs_file = fs::File::create(self.path.join("./nodes.py")).unwrap();
        let mut code = CodeLines::new();
        code.write(
            0,
            "from .common import Node,Dependency,ShapeConstraint".to_string(),
        );
        code.write(0, "from .help import ChainRef".to_string());
        code.write(0, "from .context import Context".to_string());
        code.write(0, "import taichi".to_string());
        code.write(0, "from . import operators".to_string());
        code.write(0, "def gen_nodes()->list[Node]:".to_string());
        code.write(1, "ret=[]".to_string());
        for node in &self.graph.nodes {
            match node {
                Node::Static(st) => {
                    code.write(1, "def func(context:Context):".to_string());
                    let dependency_list = st
                        .const_denpendencies
                        .nodes
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    match &st.node_type {
                        StaticNodeType::Operator(op) => {
                            let params = op
                                .input_interfaces
                                .iter()
                                .map(|a| {
                                    let keyword = a.name.clone();
                                    let varible = flaten_interface_name(a.index);
                                    format!("{keyword}=context.{varible}.get_end()")
                                })
                                .collect::<Vec<_>>()
                                .join(",");
                            match &op.operator_type {
                                operator::Type::Data(data) => {
                                    let operator_name =
                                        flaten_data_operator_name(data.upgrade().unwrap().std.id());
                                    code.write(
                                        2,
                                        format!("operators.{operator_name}.process({params})"),
                                    );
                                }
                                operator::Type::Interface(interface_operator) => {
                                    code.append(interface_operator::to_code(interface_operator), 2);
                                }
                            };
                        }
                        StaticNodeType::NewData(new) => {
                            let data_name = flaten_data_name(new.data);
                            let data = &self.graph.datas[new.data];
                            let type_name = DATA_TYPE_MAP.get_by_right(&data.data_type).unwrap();
                            let shape_name = flaten_data_name(new.shape);
                            let field_name = format!(
                                "taichi.field(dtype={type_name},shape=context.{shape_name}.value.shape)"
                            );
                            code.write(2, format!("context.{data_name}.value={field_name}"));
                        }
                        StaticNodeType::DuplicateData(dup) => (),
                        StaticNodeType::ArbitaryData(arbitary) => {
                            let data_name = flaten_data_name(arbitary.data);
                            let value_code = match &arbitary.value {
                                graph::arbitary_data::Type::Custom(code) => code,
                            };
                            code.write(2, format!("context.{data_name}.value={value_code}"));
                        }
                    };
                    code.write(
                        1,
                        format!("ret.append(Node(Dependency([{dependency_list}],func)))"),
                    );
                    code.write(1, "del func".to_string());
                }
            }
        }
        code.write(1, "return ret".to_string());
        //write to file
        let content = code.to_string();
        write!(structs_file, "{content}").unwrap();
    }
}
fn flaten_data_name(id: usize) -> String {
    format!("_elf_data_{id}")
}
fn flaten_interface_name(id: usize) -> String {
    format!("_elf_interface_{id}")
}
fn flaten_data_operator_name(id: usize) -> String {
    format!("_elf_data_operator_{id}")
}
#[test]
fn test() {
    let mut context = crate::starter::test_initialize();
    context.resource.load();
    let graph = graph::test_instance(&context.resource);
    let generator = Generator::new(&graph, "../generated/taichi1".into());
    crate::backend::Generator::generate(&generator, &context);
}
