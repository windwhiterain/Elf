use pyo3::{pyclass, pymethods};

use crate::resource::{self, plugin};

use super::{schema_tree, UIInfor};
use crate::help::key_value::KeyValue;

#[derive(Debug, Default)]
#[pyclass]
pub struct ResourceTree {
    #[pyo3(get)]
    pub plugins: Vec<Plugin>,
}
#[derive(Debug, Default, Clone)]
#[pyclass]

pub struct Plugin {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub id: usize,
    #[pyo3(get)]
    pub schemas: Vec<Schema>,
    #[pyo3(get)]
    pub data_operators: Vec<DataOperator>,
}
#[derive(Debug, Default, Clone)]
#[pyclass]

pub struct Schema {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub id: usize,
}
#[derive(Debug, Default, Clone)]
#[pyclass]

pub struct DataOperator {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
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
                .push(Schema {
                    name: schema.std.name.clone(),
                    id: schema.std.id(),
                })
        }
        for data_operator in self.plugins_content.data_operators.get_all() {
            ret.plugins[data_operator.std.plugin.upgrade().unwrap().std.id()]
                .data_operators
                .push(DataOperator {
                    name: data_operator.std.name.clone(),
                    id: data_operator.std.id(),
                })
        }
        ret
    }
}
