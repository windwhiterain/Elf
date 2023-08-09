use std::sync::Weak;

use crate::common::{self, operator::data_operator::DataOperatorR};

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
}
