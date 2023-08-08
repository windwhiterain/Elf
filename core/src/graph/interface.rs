use std::sync::Weak;

use crate::common::schema::SchemaR;

pub enum DataRef {
    Interface { index: usize, prim_offset: usize },
    Data { index: usize },
}
pub struct Interface {
    pub schema: Weak<SchemaR>,
}
