use crate::{
    backend::taichi::{type_to_code, DATA_TYPE_MAP},
    common::schema::SchemaR,
};

use super::code_line::CodeLines;

pub fn flaten_schema_name(schema: &SchemaR) -> String {
    let id = schema.std.id.load(std::sync::atomic::Ordering::Relaxed);
    format!("__elf_struct_{id}")
}
pub fn flaten_schema_ref_name(schema: &SchemaR) -> String {
    let id = schema.std.id.load(std::sync::atomic::Ordering::Relaxed);
    format!("__elf_ref_struct_{id}")
}
pub fn schema_to_struct_code(schema: &SchemaR) -> CodeLines {
    let mut code = CodeLines::new();
    //end_struct
    let struct_name = flaten_schema_name(schema);
    code.write(0, format!("class {struct_name}:"));
    code.write(1, "def __init__(self):".to_string());
    for (name, access) in schema.val.structure.get_local_prims() {
        let data_descripter = schema.val.get_data_descriptor(&access);
        let type_name = type_to_code(data_descripter);
        code.write(2, format!("self.{name} : {type_name}=None"));
    }
    for (name, access) in schema.val.structure.get_local_structs() {
        let type_name = flaten_schema_name(&schema.val.get_sub_schema(&access));
        code.write(2, format!("self.{name} : {type_name}=None"));
    }
    //ref_struct
    {
        let ref_struct_name = flaten_schema_ref_name(schema);
        code.write(0, format!("class {ref_struct_name}:"));
        code.write(1, "def __init__(self):".to_string());
        for (name, access) in schema.val.structure.get_local_prims() {
            code.write(2, format!("self.{name} : ChainRef=None"));
        }
        for (name, access) in schema.val.structure.get_local_structs() {
            code.write(2, format!("self.{name} : ChainRef=None"));
        }
        code.write(1, "def get_end(self):".to_string());
        //define get_end
        code.write(2, format!("ret={struct_name}()"));
        for (name, access) in schema.val.structure.get_local_prims() {
            code.write(2, format!("ret.{name}=self.{name}.get_end().value"));
        }
        for (name, access) in schema.val.structure.get_local_structs() {
            code.write(2, format!("ret.{name}=self.{name}.get_end()"));
        }
        code.write(2, "return ret".to_string());
    }
    //
    code
}
