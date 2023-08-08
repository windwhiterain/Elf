pub mod generator;
mod help;
mod parser;
use std::fmt::format;

use crate::{
    common::{data, schema::SchemaR, DataDescriptor, StructAccess},
    hashmap,
    resource::*,
};
use bimap::BiHashMap;
use once_cell::sync::Lazy;
use rustpython_ast::*;
use rustpython_parser::lexer::Spanned;
pub type Parser = parser::Parser;
static DATA_TYPE_MAP: Lazy<BiHashMap<&str, data::Type>> = Lazy::new(|| {
    BiHashMap::from_iter(vec![("float", data::Type::Float), ("int", data::Type::Int)].into_iter())
});
pub fn type_to_code(data: &DataDescriptor) -> &str {
    match data.dimension {
        0 => DATA_TYPE_MAP.get_by_right(&data.data_type).unwrap(),
        _ => "taichi.Field",
    }
}
