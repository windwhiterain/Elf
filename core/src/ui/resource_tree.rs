use pyo3::pyclass;

use crate::resource::{self, plugin};

use super::UIInfor;
use crate::help::key_value::KeyValue;

#[derive(Debug, Default)]
#[pyclass]
pub struct ResourceTree {
    pub plugins: Vec<Plugin>,
}
#[derive(Debug, Default)]
#[pyclass]

pub struct Plugin {
    pub name: String,
    pub id: usize,
    pub schemas: Vec<Schema>,
    pub data_operators: Vec<DataOperator>,
}
#[derive(Debug, Default)]
#[pyclass]

pub struct Schema {
    pub name: String,
    pub id: usize,
}
#[derive(Debug, Default)]
#[pyclass]

pub struct DataOperator {
    pub name: String,
    pub id: usize,
}
impl UIInfor<ResourceTree> for resource::Context {
    fn gen_infor(&self) -> ResourceTree {
        let mut ret = ResourceTree { plugins: vec![] };
        for plugin in self.plugins.get_all() {
            ret.plugins.key_value(
                plugin.std.id(),
                Plugin {
                    name: plugin.std.name.clone(),
                    id: plugin.std.id(),
                    schemas: vec![],
                    data_operators: vec![],
                },
            )
        }
        for schema in self.plugins_content.schemas.get_all() {
            ret.plugins[schema.std.plugin.upgrade().unwrap().std.id()]
                .schemas
                .key_value(
                    schema.std.id(),
                    Schema {
                        name: schema.std.name.clone(),
                        id: schema.std.id(),
                    },
                )
        }
        for data_operator in self.plugins_content.data_operators.get_all() {
            ret.plugins[data_operator.std.plugin.upgrade().unwrap().std.id()]
                .schemas
                .key_value(
                    data_operator.std.id(),
                    Schema {
                        name: data_operator.std.name.clone(),
                        id: data_operator.std.id(),
                    },
                )
        }
        ret
    }
}
