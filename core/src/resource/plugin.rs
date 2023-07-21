use std::{error::Error, fs, path::PathBuf, sync::Weak};

use super::{Directory, Resource};
use serde::Deserialize;

#[derive(Debug)]
pub enum Type {
    Taichi,
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
#[derive(Deserialize)]
struct JsonDenpendency {
    pub url: String,
}
#[derive(Debug)]
pub struct Denpendency {
    pub url: String,
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
        Resource {
            value: Directory {
                value: Plugin {
                    plugin_type: json_infor.plugin_type.try_into().unwrap(),
                    dependency: json_infor
                        .dependency
                        .into_iter()
                        .map(|js| Denpendency { url: js.url })
                        .collect(),
                    desciption: json_infor.description,
                },
                path,
            },
            name: json_infor.url,
            plugin: Weak::default(),
        }
    }
}
