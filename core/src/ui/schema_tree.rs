use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    string,
    sync::{Arc, RwLock},
};

use pyo3::{prelude::*, types::PyList};

use crate::{
    common::{structure::StructAccess, Schema},
    resource, Context,
};
#[derive(Debug, Clone)]
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
    #[pyo3(get)]
    pub childs: Vec<Node>,
    #[pyo3(get)]
    pub sc_id: i32,
}
pub fn get_node(resource: &resource::Resource<Schema>) -> Node {
    let schema = &resource.value;
    let access: StructAccess = schema.structure.clone().into();
    _get_node(
        schema,
        &schema.gen_shape_constraint_ids(),
        resource.name.clone(),
        &access,
    )
}
fn _get_node(schema: &Schema, sc_ids: &Vec<i32>, id: String, access: &StructAccess) -> Node {
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
        .map(|(id, access)| _get_node(schema, sc_ids, id.clone(), &access))
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
    m.add_submodule(sub_m)?;
    Ok(())
}
