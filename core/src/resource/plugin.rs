use std::{
    default,
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
};

use crate::{
    backend::{self, CodeFile, Parser},
    common::Schema,
    help::{ecs::Entity, file},
};

use super::{
    container::{Dir, Directory, Elem, File, Std},
    name_path::NamePath,
    PluginsContent, Resources,
};
use once_cell::sync::Lazy;
use pathdiff;
use serde::{de::IntoDeserializer, Deserialize};

pub type PluginR = crate::resource::container::File<Plugin>;
#[derive(Debug)]
pub enum Type {
    Taichi,
    Any,
}
impl TryInto<Type> for String {
    type Error = ();
    fn try_into(self) -> Result<Type, Self::Error> {
        match self.as_str() {
            "taichi" => Ok(Type::Taichi),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
pub struct Plugin {
    pub plugin_type: Type,
    pub dependency: Vec<Denpendency>,
    pub desciption: String,
}
pub static ROOT_PLUGIN: Lazy<Arc<File<Plugin>>> = Lazy::new(|| {
    Arc::new(File {
        val: Plugin::new(
            Type::Any,
            Vec::default(),
            String::from("The top level plugin that any other plugin based on"),
        ),

        std: super::container::Std::new("root".to_string(), None, true),
        dir: super::container::Dir {
            path: PathBuf::default(),
            is_local: false,
        },
    })
});
static COLD_PATH: &str = "code";
static SERIALIZED_PATH: &str = "serialized";
impl Plugin {
    pub fn new(plugin_type: Type, dependency: Vec<Denpendency>, desciption: String) -> Self {
        Plugin {
            plugin_type,
            dependency,
            desciption,
        }
    }
}
impl PluginR {
    pub fn get_code_file_node(&self) -> file::Node {
        file::Node::from(self.abs_path().join(&*COLD_PATH))
    }
}
pub fn complete(
    plugin: &Arc<PluginR>,
    plugins: &Resources<PluginR>,
    plugins_content: &mut PluginsContent,
) {
    if !plugin.std.try_complete() {
        return;
    }
    for dependency in &plugin.val.dependency {
        complete(
            plugins.find(&dependency.name_path, None).unwrap(),
            plugins,
            plugins_content,
        );
    }
    let files = plugin.get_code_file_node().get_all_child_file(".py");
    let code_files = files
        .into_iter()
        .map(|f| CodeFile {
            local_path: pathdiff::diff_paths(f.path(), plugin.abs_path()).unwrap(),
            code: f.get_code(),
        })
        .collect();
    let parser: Box<dyn backend::Parser> = match plugin.val.plugin_type {
        Type::Taichi => Box::new(backend::taichi::Parser::new()),
        Type::Any => panic!(),
    };
    parser.parse_codes(plugins_content, &plugin, code_files);
}
#[derive(Deserialize)]
struct JsonDenpendency {
    pub url: String,
}
#[derive(Debug)]
pub struct Denpendency {
    pub name_path: NamePath,
}
#[derive(Deserialize)]
struct JsonInfor {
    pub url: String,
    pub plugin_type: String,
    pub dependency: Vec<JsonDenpendency>,
    pub description: String,
}
impl From<PathBuf> for File<Plugin> {
    fn from(path: PathBuf) -> Self {
        let infor_path = path.join(PathBuf::from("./infor.json"));
        let json = fs::read(infor_path).unwrap();
        let json_infor: JsonInfor = match serde_json::from_slice(&json) {
            Ok(v) => v,
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        File::<Plugin> {
            val: Plugin::new(
                json_infor.plugin_type.try_into().unwrap(),
                json_infor
                    .dependency
                    .into_iter()
                    .map(|js| Denpendency {
                        name_path: js.url.into(),
                    })
                    .collect(),
                json_infor.description,
            ),
            std: Std::new(json_infor.url, None, false),
            dir: Dir::new(path, false),
        }
    }
}
