use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Weak;
use std::vec;

use crate::data::*;
use crate::help::*;
use crate::resource::name_path::NamePath;
use crate::structure::*;

use super::DataDescriptor;
pub type SchemaR = crate::resource::container::Elem<Schema>;
#[derive(Debug)]
pub struct Schema {
    pub structure: Arc<Structure>,
    ///Referenced by local prim offset
    data_descriptors: Vec<Descriptor>,
    ///Referenced by prim offset
    shape_constraint_refs: Vec<Option<Arc<ShapeConstraint>>>,
    ///Referenced by struct offset
    shape_constraint_maps: Vec<HashMap<String, Arc<ShapeConstraint>>>,
    ///Referenced by struct offset
    sub_schemas: Vec<Arc<SchemaR>>,
}
impl Schema {
    pub fn new<'a>(
        schemas: impl Iterator<Item = (String, &'a Arc<SchemaR>)>,
        prims: impl Iterator<Item = (String, Descriptor)>,
    ) -> Schema {
        let mut structure = Structure::new();
        let mut data_descriptors: Vec<Descriptor> = Vec::new();
        let mut shape_constraint_refs: Vec<Option<Arc<ShapeConstraint>>> = Vec::new();
        let mut shape_constraint_maps: Vec<HashMap<String, Arc<ShapeConstraint>>> = Vec::new();
        let mut sub_schemas: Vec<Arc<SchemaR>> = vec![];
        shape_constraint_maps.push(HashMap::new());
        for (id, _schema) in schemas {
            let schema = &_schema.val;
            let mut cur_sub_schemas = vec![_schema.clone()];
            cur_sub_schemas.append(&mut schema.sub_schemas.clone());
            sub_schemas.append(&mut cur_sub_schemas);
            let mut refs_t = vec(schema.shape_constraint_refs.len(), None);
            for map in &schema.shape_constraint_maps {
                let mut new_map = HashMap::new();
                let a_constraint = map.iter().next();
                match a_constraint {
                    None => continue,
                    Some((_, constraint)) => {
                        let new_constraint = deep_copy(constraint);
                        for (id, constraint) in map {
                            new_map.insert(id.clone(), new_constraint.clone());
                            for (k, v) in schema.shape_constraint_refs.iter().enumerate() {
                                match v {
                                    None => continue,
                                    Some(v) => {
                                        if constraint.as_ref().get_const_ptr()
                                            == v.as_ref().get_const_ptr()
                                        {
                                            refs_t[k] = Some(new_constraint.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                shape_constraint_maps.push(new_map);
            }
            shape_constraint_refs.append(&mut refs_t);
            structure.add_struct(id, schema.structure.clone());
        }

        for (id, data_descriptor) in prims {
            data_descriptors.push(data_descriptor);
            shape_constraint_refs.push(None);
            structure.add_prim(id);
        }
        Schema {
            structure: Arc::<Structure>::from(structure),
            data_descriptors,
            shape_constraint_refs,
            shape_constraint_maps,
            sub_schemas,
        }
    }
    pub fn get_data_descriptor(&self, access: &PrimAccess) -> &DataDescriptor {
        &if access.struct_offset == self.structure.get_self_struct().struct_offset {
            self
        } else {
            &self.get_sub_schema_by_prim(access).val
        }
        .data_descriptors[access.local_prim_offset]
    }
    pub fn get_shape_constraint_ref(&self, access: &PrimAccess) -> &Option<Arc<ShapeConstraint>> {
        &self.shape_constraint_refs[access.prim_offset]
    }
    pub fn get_shape_constraint_map(
        &self,
        access: &StructAccess,
    ) -> &HashMap<String, Arc<ShapeConstraint>> {
        &self.shape_constraint_maps[access.struct_offset]
    }
    pub fn get_shape_constraint_map_mut(
        &mut self,
        access: &StructAccess,
    ) -> &mut HashMap<String, Arc<ShapeConstraint>> {
        &mut self.shape_constraint_maps[access.struct_offset]
    }
    pub fn get_sub_schema(&self, access: &StructAccess) -> &Arc<SchemaR> {
        &self.sub_schemas[access.struct_offset - 1]
    }
    pub fn get_sub_schema_by_prim(&self, access: &PrimAccess) -> &Arc<SchemaR> {
        &self.sub_schemas[access.struct_offset - 1]
    }
    pub fn add_shape_constraint(
        &mut self,
        id: String,
        constraints: Vec<Arc<ShapeConstraint>>,
        prims: Vec<PrimAccess>,
    ) {
        let a_constraint = constraints.iter().next();
        let new_constraint = match a_constraint {
            None => {
                let a_descriptor = self.get_data_descriptor(prims.iter().next().unwrap());
                Arc::from(ShapeConstraint::new(a_descriptor.dimension))
            }
            Some(constraint) => (*constraint).clone(),
        };
        self.get_shape_constraint_map_mut(&self.structure.get_self_struct())
            .insert(id, new_constraint.clone());
        for ref_ in &mut self.shape_constraint_refs {
            match ref_ {
                None => continue,
                Some(ref_) => {
                    for constraint in &constraints {
                        if ref_.as_ref().get_const_ptr() == constraint.as_ref().get_const_ptr() {
                            assert!(ref_.dimension() == new_constraint.dimension());
                            *ref_ = new_constraint.clone();
                            break;
                        }
                    }
                }
            }
        }
        for prim in prims {
            self.shape_constraint_refs[prim.prim_offset] = Some(new_constraint.clone());
        }
    }
    pub fn add_shape_constraints(
        &mut self,
        new_constraints: impl Iterator<Item = (String, Vec<Arc<ShapeConstraint>>, Vec<PrimAccess>)>,
    ) {
        let self_map =
            &mut self.shape_constraint_maps[self.structure.get_self_struct().struct_offset];
        for (id, constraints, offsets) in new_constraints {
            self.add_shape_constraint(id, constraints, offsets);
        }
    }
    pub fn get_constraint<'a>(
        &self,
        ids: &Vec<String>,
        constraint_id: &String,
    ) -> Option<&Arc<ShapeConstraint>> {
        let end = self.structure.find_struct(ids)?;
        self.shape_constraint_maps[end.struct_offset].get(constraint_id)
    }
    ///Give each different constraints an unique i32,referenced by prim offset,used for ui or debug
    pub fn gen_shape_constraint_ids(&self) -> Vec<i32> {
        let mut class = HashMap::new();
        let mut class_count = -1;
        let mut ret = vec![];
        for ref_op in &self.shape_constraint_refs {
            let add = match ref_op {
                None => -1,
                Some(ref_) => {
                    let class_id = if !class.contains_key(&ref_.as_ref().get_const_ptr()) {
                        class_count += 1;
                        class.insert(ref_.as_ref().get_const_ptr(), class_count);
                        class_count
                    } else {
                        class[&ref_.as_ref().get_const_ptr()]
                    };
                    class_id
                }
            };
            ret.push(add)
        }
        for id in &mut ret {
            if *id == -1 {
                class_count += 1;
                *id = class_count;
            }
        }
        ret
    }
    pub fn get_self_shape_constraint_map(&self) -> &HashMap<String, Arc<ShapeConstraint>> {
        &self.shape_constraint_maps[StructAccess::root().struct_offset]
    }
}
