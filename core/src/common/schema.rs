use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Weak;
use std::vec;

use crate::data::*;
use crate::help::*;
use crate::structure::*;
#[derive(Debug)]
pub struct Schema {
    pub structure: Arc<Structure>,
    pub data_descriptors: Vec<DataDescriptor>,
    pub shape_constraint_refs: Vec<Option<Arc<ShapeConstraint>>>,
    pub shape_constraint_maps: Vec<HashMap<String, Arc<ShapeConstraint>>>,
}
impl Schema {
    pub fn new<'a>(
        schemas: impl Iterator<Item = (String, &'a Schema)>,
        prims: impl Iterator<Item = (String, DataDescriptor)>,
    ) -> Schema {
        let mut structure = Structure::new();
        let mut data_descriptors = Vec::new();
        let mut shape_constraint_refs = Vec::new();
        let mut shape_constraint_maps = Vec::new();
        shape_constraint_maps.push(HashMap::new());
        for (id, schema) in schemas {
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
                                        if eq(constraint, v) {
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
            structure.add_struct(id, Arc::<Structure>::downgrade(&schema.structure));
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
        }
    }
    pub fn add_shape_constraint(
        &mut self,
        id: String,
        constraints: Vec<&Arc<ShapeConstraint>>,
        prim_offsets: Vec<usize>,
    ) {
        let access: StructAccess = self.structure.clone().into();
        let self_map = &mut self.shape_constraint_maps[access.struct_offset()];
        let a_constraint = constraints.iter().next();
        let new_constraint = match a_constraint {
            None => {
                let a_descriptor = &self.data_descriptors[*prim_offsets.iter().next().unwrap()];
                Arc::from(ShapeConstraint::new(a_descriptor.dimension))
            }
            Some(constraint) => (**constraint).clone(),
        };
        self_map.insert(id, new_constraint.clone());
        for ref_ in &mut self.shape_constraint_refs {
            match ref_ {
                None => continue,
                Some(ref_) => {
                    for constraint in &constraints {
                        if eq(ref_, constraint) {
                            assert!(ref_.dimension() == new_constraint.dimension());
                            *ref_ = new_constraint.clone();
                            break;
                        }
                    }
                }
            }
        }
        for offset in prim_offsets {
            self.shape_constraint_refs[offset] = Some(new_constraint.clone());
        }
    }
    pub fn add_shape_constraints<'a>(
        &mut self,
        new_constraints: impl Iterator<Item = (String, Vec<&'a Arc<ShapeConstraint>>, Vec<usize>)>,
    ) {
        let access: StructAccess = self.structure.clone().into();
        let self_map = &mut self.shape_constraint_maps[access.struct_offset()];
        for (id, constraints, offsets) in new_constraints {
            self.add_shape_constraint(id, constraints, offsets);
        }
    }
    pub fn get_constraint<'a>(
        &self,
        ids: impl Iterator<Item = &'a String>,
        constraint_id: &String,
    ) -> Option<&Arc<ShapeConstraint>> {
        let root: StructAccess = self.structure.clone().into();
        let end = root.access(ids);
        self.shape_constraint_maps[end?.struct_offset()].get(constraint_id)
    }
    pub fn gen_shape_constraint_ids(&self) -> Vec<i32> {
        let mut class = HashMap::new();
        let mut class_count = -1;
        let mut ret = vec![];
        for ref_op in &self.shape_constraint_refs {
            let add = match ref_op {
                None => -1,
                Some(ref_) => {
                    let class_id = if !class.contains_key(&ptr(ref_)) {
                        class_count += 1;
                        class.insert(ptr(ref_), class_count);
                        class_count
                    } else {
                        class[&ptr(ref_)]
                    };
                    class_id
                }
            };
            ret.push(add)
        }
        ret
    }
}
