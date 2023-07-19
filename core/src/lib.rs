pub mod common;
pub mod help;
pub mod python;
pub mod resource;
mod starter;
pub mod ui;
pub mod ui_debug;
use pyo3::prelude::*;
use std::{collections::HashMap, iter, path::PathBuf, sync::Arc, sync::Weak};

use common::*;
///The only context for an elf applycation
#[pyclass(unsendable)]
pub struct Context {
    ///resource context
    pub resource: resource::Context,
    ///python interpreter
    pub py_context: python::Context,
}

#[pymethods]
impl Context {
    #[new]
    pub fn new(py_context: python::Context) -> Context {
        Context {
            resource: resource::Context::new(),
            py_context,
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
        let arc_light = Arc::from(resource::Resource::new("light".into(), light));
        ui_debug::show_schema(&arc_light, self);
        self.resource.schemas.add(arc_light);
    }
}
#[pymodule]
#[pyo3(name = "elf_py")]
fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Context>()?;
    ui::gen_module(py, m)?;
    python::gen_module(py, m)?;
    Ok(())
}
