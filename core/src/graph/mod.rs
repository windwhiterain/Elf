pub mod data_duplication;
pub mod dependency;
pub mod interface;
pub mod new_arbitary_data;
pub mod node;
pub mod operator;
use crate::{
    common::{data, schema::SchemaR},
    resource::{
        self,
        name_path::NamePath,
        plugin::{Denpendency, PluginR, ROOT_PLUGIN},
    },
};
use node::{Node, StaticNode, StaticNodeType};
use std::{
    ops::Deref,
    sync::{Arc, Weak},
};

use self::{
    data_duplication::DuplicateData,
    interface::{DataRef, Interface},
    operator::{InputInterface, Operator},
};
pub struct Graph {
    pub plugins: Vec<Arc<PluginR>>,
    pub datas: Vec<data::Descriptor>,
    pub interfaces: Vec<Interface>,
    pub nodes: Vec<Node>,
}

///temp
pub fn test_instance<'a>(resource: &resource::Context) -> Graph {
    let test_plugin1 = resource
        .plugins
        .find(&"test_plugin1".to_string().into(), None)
        .unwrap();
    let test_plugin2 = resource
        .plugins
        .find(&"test_plugin2".to_string().into(), None)
        .unwrap();
    let plugins = vec![
        test_plugin1.clone(),
        test_plugin2.clone(),
        ROOT_PLUGIN.deref().clone(),
    ];
    let datas = vec![
        data::Descriptor {
            dimension: 1,
            data_type: data::Type::Int,
        },
        data::Descriptor {
            dimension: 1,
            data_type: data::Type::Float,
        },
    ];
    let interfaces = vec![Interface {
        schema: Arc::downgrade(
            resource
                .plugins_content
                .schemas
                .find(&"Complex".into(), Some(&test_plugin2))
                .unwrap(),
        ),
    }];
    let nodes = vec![Node::Static(StaticNode {
        node_type: StaticNodeType::Operator(Operator {
            operator_type: operator::Type::Data(Arc::downgrade(
                resource
                    .plugins_content
                    .data_operators
                    .find(&"Modify".into(), Some(&test_plugin1))
                    .unwrap(),
            )),
            input_interfaces: vec![InputInterface {
                name: "complex".to_string(),
                index: 0,
            }],
        }),
        const_denpendencies: dependency::Const { nodes: vec![] },
    })];
    Graph {
        plugins,
        datas,
        interfaces,
        nodes,
    }
}
