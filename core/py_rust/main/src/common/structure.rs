use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
pub struct Field {
    value: Weak<Structure>,
    offset_N: i32,
    offset_T: i32,
}
pub struct NStruct {
    fields: HashMap<String, Field>,
    count_N: i32,
    count_T: i32,
}
pub enum Structure {
    T(()),
    N(NStruct),
}
impl NStruct {
    fn new(fields: impl Iterator<Item = (String, Weak<Structure>)>) -> NStruct {
        let mut offset_N = 1;
        let mut offset_T = 0;
        let mut fields_with_offset = HashMap::<String, Field>::new();
        for (id, value_weak) in fields {
            let value = value_weak.upgrade().unwrap();
            fields_with_offset.insert(
                id,
                Field {
                    value: value_weak,
                    offset_N,
                    offset_T,
                },
            );
            offset_N += value.count_N();
            offset_T += value.count_T();
        }
        NStruct {
            fields: fields_with_offset,
            count_N: offset_N,
            count_T: offset_T,
        }
    }
    pub fn sub_struct(&self, id: String) -> Option<SubStruct> {
        let field = self.fields.get(&id)?;
        Some(SubStruct {
            structure: field.value.clone(),
            offset_T: field.offset_T,
            offset_N: field.offset_N,
        })
    }
}
impl Structure {
    pub fn new_T() -> Structure {
        Structure::T(())
    }
    pub fn new_N(fields: impl Iterator<Item = (String, Weak<Structure>)>) -> Structure {
        Structure::N(NStruct::new(fields))
    }
    pub fn count_N(&self) -> i32 {
        match self {
            Structure::N(n) => n.count_N,
            Structure::T(t) => 0,
        }
    }
    pub fn count_T(&self) -> i32 {
        match self {
            Structure::N(n) => n.count_T,
            Structure::T(t) => 1,
        }
    }
}
pub struct SubStruct {
    structure: Weak<Structure>,
    offset_T: i32,
    offset_N: i32,
}
