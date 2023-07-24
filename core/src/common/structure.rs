use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::resource::name_path::NamePath;
#[derive(Debug, Clone)]
pub struct StructField {
    value: Weak<Structure>,
    struct_offset: usize,
    prim_offset: usize,
    uniq_offset: usize,
}
#[derive(Debug, Clone)]
pub struct PrimField {
    pub prim_offset: usize,
    pub uniq_offset: usize,
}
#[derive(Debug)]
pub struct Structure {
    struct_fields: HashMap<String, StructField>,
    prim_fields: HashMap<String, PrimField>,
    pub struct_count: usize,
    pub prim_count: usize,
}
pub struct ConstructingStructure {
    struct_fields: HashMap<String, StructField>,
    prim_fields: HashMap<String, PrimField>,
    pub struct_count: usize,
    pub prim_count: usize,
}
impl Structure {
    pub fn new() -> Structure {
        Structure {
            struct_fields: HashMap::new(),
            prim_fields: HashMap::new(),
            struct_count: 1,
            prim_count: 0,
        }
    }
    pub fn from(
        struct_fields: impl Iterator<Item = (String, Weak<Structure>)>,
        prim_fields: impl Iterator<Item = String>,
    ) -> Structure {
        let mut ret = Structure::new();
        for (id, structure) in struct_fields {
            ret.add_struct(id, structure)
        }
        for id in prim_fields {
            ret.add_prim(id)
        }
        ret
    }
    pub fn add_struct(&mut self, id: String, structure: Weak<Structure>) {
        let value = structure.upgrade().unwrap();
        self.struct_fields.insert(
            id,
            StructField {
                value: structure,
                struct_offset: self.struct_count,
                prim_offset: self.prim_count,
                uniq_offset: self.uniq_struct_count(),
            },
        );
        self.struct_count += value.struct_count;
        self.prim_count += value.prim_count;
    }
    pub fn add_prim(&mut self, id: String) {
        self.prim_fields.insert(
            id,
            PrimField {
                prim_offset: self.prim_count,
                uniq_offset: self.uniq_prim_count(),
            },
        );
        self.prim_count += 1;
    }
    pub fn get_struct(&self, id: &String) -> Option<StructField> {
        Some((*(self.struct_fields.get(id)?)).clone())
    }
    pub fn get_prim(&self, id: &String) -> Option<PrimField> {
        Some((*(self.prim_fields.get(id)?)).clone())
    }
    pub fn uniq_struct_count(&self) -> usize {
        self.struct_fields.len()
    }
    pub fn uniq_prim_count(&self) -> usize {
        self.prim_fields.len()
    }
}
///Use this to access structure's data,this struct associated with a sub structure in a real structure
#[derive(Debug)]
pub struct StructAccess {
    structure: Arc<Structure>,
    struct_offset: usize,
    prim_offset: usize,
    uniq_offset: usize,
}
impl StructAccess {
    pub fn new(structure: Arc<Structure>) -> StructAccess {
        StructAccess {
            structure,
            struct_offset: 0,
            prim_offset: 0,
            uniq_offset: 0,
        }
    }
    pub fn get_struct_offset(&self) -> usize {
        self.struct_offset
    }
    pub fn get_uniq_offset(&self) -> usize {
        self.uniq_offset
    }
    pub fn find_uniq_offset(&self, id: &String) -> Option<usize> {
        Some(self.structure.get_prim(id)?.uniq_offset + self.uniq_offset)
    }
    pub fn find_prim_offset(&self, id: &String) -> Option<usize> {
        Some(self.structure.get_prim(id)?.prim_offset + self.prim_offset)
    }
    ///Get a sub accessor by the struct field of current sub structure
    pub fn get_struct_by_struct_field(&self, field: &StructField) -> StructAccess {
        StructAccess {
            structure: field.value.upgrade().unwrap().clone(),
            struct_offset: self.struct_offset + field.struct_offset,
            prim_offset: self.prim_offset + field.prim_offset,
            uniq_offset: self.uniq_offset + field.uniq_offset,
        }
    }
    ///Get a offseted prim field by the prim field of current sub structure
    pub fn get_prim_by_prim_field(&self, field: &PrimField) -> PrimField {
        PrimField {
            prim_offset: self.prim_offset + field.prim_offset,
            uniq_offset: self.uniq_offset + field.uniq_offset,
        }
    }
    ///Get a sub accessor by a name path to it,from the current sub structure
    pub fn find_struct_by_strings<'a>(
        &self,
        ids: impl Iterator<Item = &'a String>,
    ) -> Option<StructAccess> {
        let mut struct_offset = self.struct_offset;
        let mut prim_offset = self.prim_offset;
        let mut uniq_offset = self.uniq_offset;
        let mut cur = self.structure.clone();
        for id in ids {
            let field = cur.get_struct(id)?;
            struct_offset += field.struct_offset;
            prim_offset += field.prim_offset;
            uniq_offset += field.uniq_offset;
            cur = field.value.upgrade().unwrap().clone();
        }
        Some(StructAccess {
            structure: cur,
            struct_offset,
            prim_offset,
            uniq_offset,
        })
    }
    pub fn find_struct(&self, name_path: &NamePath) -> Option<StructAccess> {
        self.find_struct_by_strings(name_path.prefixs())
    }
    ///Get all sub struture from current sub structure's struct fields
    pub fn get_structs(&self) -> impl Iterator<Item = (&String, StructAccess)> + '_ {
        self.structure
            .struct_fields
            .iter()
            .map(|(id, field)| (id, self.get_struct_by_struct_field(field)))
    }
    ///Get all sub struture from current sub structure's struct fields
    pub fn get_prims(&self) -> impl Iterator<Item = (&String, PrimField)> {
        self.structure
            .prim_fields
            .iter()
            .map(|(id, field)| (id, self.get_prim_by_prim_field(field)))
    }
}
impl From<&Arc<Structure>> for StructAccess {
    fn from(structure: &Arc<Structure>) -> Self {
        StructAccess::new(structure.clone())
    }
}
