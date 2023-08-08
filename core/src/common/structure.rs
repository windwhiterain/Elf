use std::{
    collections::HashMap,
    sync::{Arc, Weak},
    thread::AccessError,
};

use crate::resource::name_path::{self, NamePath};
#[derive(Debug, Clone)]
pub struct StructField {
    structure: Arc<Structure>,
    struct_offset: usize,
    prim_offset: usize,
    uniq_offset: usize,
}
#[derive(Debug, Clone)]
pub struct PrimField {
    pub prim_offset: usize,
    pub uniq_offset: usize,
    pub local_offset: usize,
}
#[derive(Debug)]
pub struct Structure {
    struct_fields: HashMap<String, StructField>,
    prim_fields: HashMap<String, PrimField>,
    struct_count: usize,
    prim_count: usize,
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
        struct_fields: impl Iterator<Item = (String, Arc<Structure>)>,
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
    pub fn add_struct(&mut self, id: String, structure: Arc<Structure>) {
        self.struct_fields.insert(
            id,
            StructField {
                structure: structure.clone(),
                struct_offset: self.struct_count,
                prim_offset: self.prim_count,
                uniq_offset: self.local_struct_count(),
            },
        );
        self.struct_count += structure.struct_count;
        self.prim_count += structure.prim_count;
    }
    pub fn add_prim(&mut self, id: String) {
        self.prim_fields.insert(
            id,
            PrimField {
                prim_offset: self.prim_count,
                uniq_offset: self.local_prim_count(),
                local_offset: self.prim_fields.len(),
            },
        );
        self.prim_count += 1;
    }
    pub fn get_self_struct(&self) -> StructAccess {
        StructAccess::root()
    }
    pub fn find_local_struct(&self, id: &String) -> Option<StructAccess> {
        Some(StructAccess::from(self.struct_fields.get(id)?))
    }
    pub fn find_local_prim(&self, id: &String) -> Option<PrimAccess> {
        Some(StructAccess::root().access_prim_field(self.prim_fields.get(id)?))
    }
    pub fn local_struct_count(&self) -> usize {
        self.struct_fields.len()
    }
    pub fn local_prim_count(&self) -> usize {
        self.prim_fields.len()
    }
    pub fn struct_count(&self) -> usize {
        self.struct_count
    }
    pub fn prim_count(&self) -> usize {
        self.prim_count
    }
    pub fn get_local_structs(&self) -> impl Iterator<Item = (&String, StructAccess)> + '_ {
        self.get_local_struct_fields()
            .map(|(name, field)| (name, self.get_self_struct().access_struct_field(field)))
    }
    pub fn get_local_prims(&self) -> impl Iterator<Item = (&String, PrimAccess)> {
        self.get_local_prim_fields()
            .map(|(name, field)| (name, self.get_self_struct().access_prim_field(field)))
    }
    pub fn get_local_struct_fields(&self) -> impl Iterator<Item = (&String, &StructField)> {
        self.struct_fields.iter()
    }
    pub fn get_local_prim_fields(&self) -> impl Iterator<Item = (&String, &PrimField)> {
        self.prim_fields.iter()
    }
    fn find_struct_raw<'a>(&self, names: &NamePath) -> Option<(StructAccess, &Structure)> {
        let mut access = StructAccess::root();
        let mut cur = self;
        for id in names.all() {
            let field = cur.struct_fields.get(id)?;
            access = access.access_struct_field(&field);
            cur = field.structure.as_ref();
        }
        Some((access, cur))
    }
    pub fn find_struct<'a>(&self, names: &NamePath) -> Option<StructAccess> {
        Some(self.find_struct_raw(names)?.0)
    }
    pub fn find_prim(&self, names: &NamePath) -> Option<PrimAccess> {
        let (struct_access, structure) = self.find_struct_raw(&names.prefixs())?;
        Some(struct_access.access_prim_field(structure.prim_fields.get(names.name())?))
    }
}
///Use this to access structure's data,this struct associated with a sub structure in a real structure
#[derive(Debug)]
pub struct StructAccess {
    pub struct_offset: usize,
    pub prim_offset: usize,
    pub uniq_offset: usize,
}
impl StructAccess {
    pub fn root() -> StructAccess {
        StructAccess {
            struct_offset: 0,
            prim_offset: 0,
            uniq_offset: 0,
        }
    }
    pub fn access_struct_field(&self, field: &StructField) -> StructAccess {
        StructAccess {
            struct_offset: self.struct_offset + field.struct_offset,
            prim_offset: self.prim_offset + field.prim_offset,
            uniq_offset: self.uniq_offset + field.uniq_offset,
        }
    }
    pub fn access_prim_field(&self, field: &PrimField) -> PrimAccess {
        PrimAccess {
            struct_offset: self.struct_offset,
            prim_offset: self.prim_offset + field.prim_offset,
            uniq_offset: self.uniq_offset + field.uniq_offset,
            local_prim_offset: field.local_offset,
        }
    }
}
impl From<&StructField> for StructAccess {
    fn from(field: &StructField) -> Self {
        StructAccess {
            struct_offset: field.struct_offset,
            prim_offset: field.prim_offset,
            uniq_offset: field.uniq_offset,
        }
    }
}
#[derive(Debug)]
pub struct PrimAccess {
    pub struct_offset: usize,
    pub prim_offset: usize,
    pub uniq_offset: usize,
    pub local_prim_offset: usize,
}
#[derive(Debug)]
pub struct View {
    pub structure: Arc<Structure>,
    pub access: StructAccess,
}
impl View {
    pub fn get_local_structs(&self) -> impl Iterator<Item = (&String, View)> {
        self.structure
            .get_local_struct_fields()
            .map(|(name, field)| {
                (
                    name,
                    View {
                        structure: field.structure.clone(),
                        access: self.access.access_struct_field(field),
                    },
                )
            })
    }
    pub fn get_local_prims(&self) -> impl Iterator<Item = (&String, PrimAccess)> {
        self.structure
            .get_local_prim_fields()
            .map(|(name, field)| (name, self.access.access_prim_field(field)))
    }
}
impl From<Arc<Structure>> for View {
    fn from(value: Arc<Structure>) -> Self {
        Self {
            structure: value,
            access: StructAccess::root(),
        }
    }
}
