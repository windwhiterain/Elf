use super::{
    arbitary_data::ArbitaryData, data_duplication::DuplicateData, dependency, new_data::NewData,
    operator::Operator,
};

pub enum StaticNodeType {
    Operator(Operator),
    NewData(NewData),
    DuplicateData(DuplicateData),
    ArbitaryData(ArbitaryData),
}
pub struct StaticNode {
    pub node_type: StaticNodeType,
    pub const_denpendencies: dependency::Const,
}
pub enum Node {
    Static(StaticNode),
}
