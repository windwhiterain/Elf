pub mod arbitary_data;
pub mod data_duplication;
pub mod dependency;
pub mod help;
pub mod interface;
pub mod new_data;
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
    help::{custom_arbitary_data, custom_data, new_data},
    interface::{DataRef, Interface},
    operator::{InputInterface, InterfaceOperator, Link, Operator},
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
        .find(&vec!["test_plugin1".to_string()], None)
        .unwrap();
    let test_plugin2 = resource
        .plugins
        .find(&vec!["test_plugin2".to_string()], None)
        .unwrap();
    let plugins = vec![
        test_plugin1.clone(),
        test_plugin2.clone(),
        ROOT_PLUGIN.deref().clone(),
    ];
    let complex = resource
        .plugins_content
        .schemas
        .find(&vec!["Complex".to_string()], Some(&test_plugin2))
        .unwrap();
    let datas = vec![
        data::Descriptor {
            dimension: 2,
            data_type: data::Type::Float,
        },
        data::Descriptor {
            dimension: 2,
            data_type: data::Type::Float,
        },
        data::Descriptor {
            dimension: 2,
            data_type: data::Type::Int,
        },
        data::Descriptor {
            dimension: 0,
            data_type: data::Type::Int,
        },
        custom_data("ShapeConstraint".to_string(), 0),
    ];
    let interfaces = vec![Interface {
        schema: Arc::downgrade(complex),
    }];
    let nodes = vec![
        custom_arbitary_data(4, "ShapeConstraint(shape=(2,2))".to_string(), vec![]),
        new_data(0, 4, vec![0]),
        new_data(1, 4, vec![0]),
        new_data(2, 4, vec![0]),
        Node::Static(StaticNode {
            node_type: StaticNodeType::Operator(Operator {
                operator_type: operator::Type::Interface(InterfaceOperator {
                    interface: 0,
                    links: vec![
                        Link {
                            from: operator::DataFrom::Data(0),
                            to: vec!["ff".to_string(), "a".to_string()],
                        },
                        Link {
                            from: operator::DataFrom::Data(1),
                            to: vec!["ff".to_string(), "b".to_string()],
                        },
                        Link {
                            from: operator::DataFrom::Data(2),
                            to: vec!["ints".to_string()],
                        },
                        Link {
                            from: operator::DataFrom::Data(3),
                            to: vec!["mod".to_string()],
                        },
                        Link {
                            from: operator::DataFrom::Data(4),
                            to: vec!["line_sc".to_string()],
                        },
                    ],
                }),
                input_interfaces: vec![],
            }),
            const_denpendencies: dependency::Const {
                nodes: vec![1, 2, 3],
            },
        }),
        Node::Static(StaticNode {
            node_type: StaticNodeType::Operator(Operator {
                operator_type: operator::Type::Data(Arc::downgrade(
                    resource
                        .plugins_content
                        .data_operators
                        .find(&vec!["Modify".to_string()], Some(&test_plugin1))
                        .unwrap(),
                )),
                input_interfaces: vec![InputInterface {
                    name: "complex".to_string(),
                    index: 0,
                }],
            }),
            const_denpendencies: dependency::Const { nodes: vec![4] },
        }),
    ];
    Graph {
        plugins,
        datas,
        interfaces,
        nodes,
    }
}
