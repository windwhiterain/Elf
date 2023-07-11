use std::{string, sync::Arc};

use pyo3::prelude::*;

use crate::{
    common::{structure::StructAccess, Schema},
    resource, Context,
};
#[derive(Debug)]
#[pyclass]
pub struct Node {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub w: f32,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get, set)]
    pub childs: Vec<Node>,
    #[pyo3(get)]
    pub sc_id: i32,
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            x: self.x,
            y: self.y,
            w: self.w,
            name: self.name.clone(),
            childs: self.childs.clone(),
            sc_id: self.sc_id,
        }
    }
}
#[pyfunction]
pub fn get_current_node(context: &Context) -> Node {
    let resource = &context
        .resource
        .schemas
        .get(context.current_schema)
        .unwrap()
        .clone();
    let schema = &resource.value;
    let access: StructAccess = schema.structure.clone().into();
    get_node(
        schema,
        &schema.gen_shape_constraint_ids(),
        resource.name.clone(),
        &access,
    )
}
fn get_node(schema: &Schema, sc_ids: &Vec<i32>, id: String, access: &StructAccess) -> Node {
    let mut childs = access
        .prims()
        .map(|(id, field)| Node {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            name: id.clone(),
            childs: vec![],
            sc_id: sc_ids[field.prim_offset],
        })
        .collect::<Vec<Node>>();
    let mut subs = access
        .subs()
        .map(|(id, access)| get_node(schema, sc_ids, id.clone(), &access))
        .collect::<Vec<Node>>();
    childs.append(&mut subs);
    Node {
        x: 0.0,
        y: 0.0,
        w: 0.0,
        name: id,
        childs,
        sc_id: -2,
    }
}
pub fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    let sub_m = PyModule::new(py, "schema_tree")?;
    sub_m.add_class::<Node>()?;
    sub_m.add_function(wrap_pyfunction!(get_current_node, m)?)?;
    m.add_submodule(sub_m)?;
    Ok(())
}
