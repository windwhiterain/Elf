pub mod generator;
mod help;
mod parser;
use std::fmt::format;

use crate::{
    common::{data, schema::SchemaR, StructAccess},
    hashmap,
    resource::*,
};
use bimap::BiHashMap;
use once_cell::sync::Lazy;
use rustpython_ast::*;
pub type Parser = parser::Parser;
static DATA_TYPE_MAP: Lazy<BiHashMap<&str, data::Type>> = Lazy::new(|| {
    BiHashMap::from_iter(vec![("float", data::Type::Float), ("int", data::Type::Int)].into_iter())
});
pub fn schema_to_struct_code(schema: &SchemaR) -> Vec<String> {
    let mut lines = vec![];
    let id = schema.std.id.load(std::sync::atomic::Ordering::Relaxed);
    lines.push(format!("class __elf_struct_{id}"));
    for (name, access) in schema.val.structure.get_local_prims() {
        let type_name = DATA_TYPE_MAP
            .get_by_right(&schema.val.get_data_descriptor(&access).data_type)
            .unwrap();
        lines.push(format!("    {name} : {type_name}"));
    }
    for (name, access) in schema.val.structure.get_local_structs() {
        let type_name = &schema.val.get_sub_schema(&access).std.name;
        lines.push(format!("    {name} : {type_name}"));
    }
    lines
}
