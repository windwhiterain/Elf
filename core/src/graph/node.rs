use super::{
    data_duplication::DuplicateData, dependency, new_arbitary_data::NewArbitaryData,
    operator::Operator,
};

pub enum StaticNodeType {
    Operator(Operator),
    NewArbitryData(NewArbitaryData),
    DuplicateData(DuplicateData),
}
pub struct StaticNode {
    pub node_type: StaticNodeType,
    pub const_denpendencies: dependency::Const,
}
pub enum Node {
    Static(StaticNode),
}
