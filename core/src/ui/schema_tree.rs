use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    string,
    sync::{Arc, RwLock},
};

use pyo3::{prelude::*, types::PyList};

use crate::{
    common::{
        structure::{self, StructAccess},
        Schema, Structure,
    },
    help::ecs::Entity,
    resource::{self, container::Std},
    Context,
};

use super::UIInfor;
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
impl UIInfor<Node> for resource::container::Elem<Schema> {
    fn gen_infor(&self) -> Node {
        let view = &structure::View::from(self.val.structure.clone());
        _get_node(
            &self.val,
            &self.val.gen_shape_constraint_ids(),
            self.std.name.clone(),
            view,
        )
    }
}
fn _get_node(schema: &Schema, sc_ids: &Vec<i32>, id: String, access: &structure::View) -> Node {
    let mut childs = access
        .get_local_prims()
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
        .get_local_structs()
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
