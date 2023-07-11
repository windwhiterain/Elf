pub mod common;
pub mod help;
pub mod python;
pub mod resource;
pub mod ui;
use pyo3::prelude::*;
use std::{collections::HashMap, iter, sync::Arc, sync::Weak};

use common::*;
///The only context for an elf applycation
#[pyclass(unsendable)]
pub struct Context {
    ///resource context
    pub resource: resource::Context,
    ///temp
    pub current_schema: usize,
}

#[pymethods]
impl Context {
    #[new]
    pub fn new() -> Context {
        Context {
            resource: resource::Context::new(),
            current_schema: 0,
        }
    }
    ///temp
    #[pyo3(signature = ())]
    pub fn init(&mut self) {
        let mut ray = Schema::new(
            iter::empty(),
            [
                (String::from("dir"), DataDescriptor { dimension: 1 }),
                (String::from("ori"), DataDescriptor { dimension: 1 }),
            ]
            .into_iter(),
        );
        let mut dens = Schema::new(
            iter::empty(),
            [(String::from("arr"), DataDescriptor { dimension: 1 })].into_iter(),
        );
        ray.add_shape_constraint("all".into(), [].into(), [0, 1].into());
        dens.add_shape_constraint("all".into(), [].into(), [0].into());
        let mut light = Schema::new(
            [
                ("r1".into(), &ray),
                ("r2".into(), &ray),
                ("d1".into(), &dens),
            ]
            .into_iter(),
            [("more".into(), DataDescriptor { dimension: 1 })].into_iter(),
        );
        let sc1 = light
            .get_constraint([&"r1".to_string()].into_iter(), &"all".to_string())
            .unwrap()
            .clone();
        let sc2 = light
            .get_constraint([&"r2".to_string()].into_iter(), &"all".to_string())
            .unwrap()
            .clone();

        light.add_shape_constraint("sc1".into(), [&sc1, &sc2].into(), [].into());
        self.resource
            .schemas
            .add(Arc::from(resource::Resource::new("ray".into(), ray)));
        self.resource
            .schemas
            .add(Arc::from(resource::Resource::new("dens".into(), dens)));
        self.current_schema = self
            .resource
            .schemas
            .add(Arc::from(resource::Resource::new("light".into(), light)));
    }
}
#[pymodule]
#[pyo3(name = "elf_py")]
fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>()?;
    ui::gen_module(py, m)?;
    Ok(())
}
#[test]
fn test() {
    let mut context = Context::new();
    context.init();
    let root = ui::schema_tree::get_current_node(&context);
    for child in &root.childs {
        println!("{:?}", child)
    }
}
