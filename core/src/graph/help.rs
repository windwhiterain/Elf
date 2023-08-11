use crate::common::{data, DataDescriptor};

use super::{
    arbitary_data,
    arbitary_data::ArbitaryData,
    dependency,
    new_data::NewData,
    node::{Node, StaticNode, StaticNodeType},
};

pub fn new_data(data: usize, shape: usize, const_denpendent_nodes: Vec<usize>) -> Node {
    Node::Static(StaticNode {
        node_type: StaticNodeType::NewData(NewData { data, shape }),
        const_denpendencies: dependency::Const {
            nodes: const_denpendent_nodes,
        },
    })
}
pub fn data(dtype: data::Type, dim: usize) -> DataDescriptor {
    DataDescriptor {
        data_type: dtype,
        dimension: dim,
    }
}
pub fn custom_data(dtype: String, dim: usize) -> DataDescriptor {
    DataDescriptor {
        data_type: data::Type::Custom(dtype),
        dimension: dim,
    }
}
pub fn custom_arbitary_data(
    data: usize,
    value: String,
    const_denpendent_nodes: Vec<usize>,
) -> Node {
    Node::Static(StaticNode {
        node_type: StaticNodeType::ArbitaryData(ArbitaryData {
            data,
            value: arbitary_data::Type::Custom(value),
        }),
        const_denpendencies: dependency::Const {
            nodes: const_denpendent_nodes,
        },
    })
}
