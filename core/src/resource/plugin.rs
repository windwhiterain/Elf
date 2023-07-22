use std::{
    default,
    error::Error,
    fs,
    path::PathBuf,
    sync::{Arc, Weak},
};

use crate::{
    common::Schema,
    frontend::{self, Parser},
    help::file,
};

use super::{Directory, NamePath, PluginsContent, Resource, Resources};
use once_cell::sync::Lazy;
use serde::{de::IntoDeserializer, Deserialize};

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
pub static ROOT_PLUGIN: Lazy<Arc<Resource<Directory<Plugin>>>> = Lazy::new(|| {
    Arc::new(Resource {
        name: "root".to_string(),
        value: Directory::new(
            Plugin::new(
                Type::Any,
                Vec::default(),
                String::from("The top level plugin that any other plugin based on"),
            ),
            PathBuf::default(),
        ),
        plugin: Weak::default(),
        completed: true.into(),
    })
});
static COLD_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("./code"));
static SERIALIZED_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("./serialized"));
impl Plugin {
    pub fn new(plugin_type: Type, dependency: Vec<Denpendency>, desciption: String) -> Self {
        Plugin {
            plugin_type,
            dependency,
            desciption,
        }
    }
}
impl Directory<Plugin> {
    pub fn get_code_file_node(&self) -> file::Node {
        file::Node::from(self.path.join(&*COLD_PATH))
    }
}
pub fn complete(
    plugin: &Arc<Resource<Directory<Plugin>>>,
    plugins: &Resources<Directory<Plugin>>,
    plugins_content: &mut PluginsContent,
) {
    if !plugin.try_complete() {
        return;
    }
    for dependency in &plugin.value.value.dependency {
        complete(
            plugins.find(&dependency.name_path, None).unwrap(),
            plugins,
            plugins_content,
        );
    }
    let codes = plugin.value.get_code_file_node().get_all_code(".py");
    let parser: Box<dyn frontend::Parser> = match plugin.value.value.plugin_type {
        Type::Taichi => Box::new(frontend::taichi::Parser::new()),
        Type::Any => panic!(),
    };
    parser.parse_codes(plugins_content, &plugin, codes);
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
impl From<PathBuf> for Resource<Directory<Plugin>> {
    fn from(path: PathBuf) -> Self {
        let infor_path = path.join(PathBuf::from("./infor.json"));
        let json = fs::read(infor_path).unwrap();
        let json_infor: JsonInfor = match serde_json::from_slice(&json) {
            Ok(v) => v,
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        Resource::new(
            json_infor.url,
            Directory {
                value: Plugin::new(
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
                path,
            },
            None,
            false,
        )
    }
}
