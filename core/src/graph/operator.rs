use std::sync::Weak;

use crate::common::{self, operator::data_operator::DataOperatorR, structure::PrimAccess};

use super::dependency;

pub struct Operator {
    pub operator_type: Type,
    pub input_interfaces: Vec<InputInterface>,
}
pub struct InputInterface {
    pub name: String,
    pub index: usize,
}
pub enum Type {
    Data(Weak<DataOperatorR>),
    Interface(InterfaceOperator),
}
pub enum DataFrom {
    Data(usize),
    Interface { name: String, prim: PrimAccess },
}
pub struct Link {
    pub from: DataFrom,
    pub to: Vec<String>,
}
pub struct InterfaceOperator {
    pub interface: usize,
    pub links: Vec<Link>,
}
