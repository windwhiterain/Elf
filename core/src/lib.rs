pub mod common;
pub mod help;
pub mod python;
pub mod ui;
use pyo3::prelude::*;
use std::{collections::HashMap, iter, sync::Arc, sync::Weak};

use common::*;
#[pyclass(unsendable)]
pub struct Context {
    pub resources: Resources,
    pub current_schema: String,
}
pub struct Resource<T> {
    pub value: T,
}
impl<T> Resource<T> {
    pub fn new(value: T) -> Resource<T> {
        Resource { value }
    }
}
pub struct Resources {
    pub schemas: HashMap<String, Arc<Resource<Schema>>>,
}
impl Resources {
    pub fn new() -> Resources {
        {
            Resources {
                schemas: HashMap::new(),
            }
        }
    }
}
#[pymethods]
impl Context {
    #[new]
    pub fn new() -> Context {
        Context {
            resources: Resources::new(),
            current_schema: String::new(),
        }
    }
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

        light.add_shape_constraint("sc1".into(), [&sc1, &sc2].into(), [5].into());
        self.resources
            .schemas
            .insert("ray".into(), Arc::from(Resource::new(ray)));
        self.resources
            .schemas
            .insert("dens".into(), Arc::from(Resource::new(dens)));
        self.resources
            .schemas
            .insert("light".into(), Arc::from(Resource::new(light)));
        self.current_schema = String::from("light");
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
