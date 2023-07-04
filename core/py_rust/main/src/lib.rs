pub mod common;
pub mod help;
pub mod ui;
use std::{collections::HashMap, iter, rc::Weak, sync::Arc};

use common::*;
pub struct Context {
    resources: Resources,
}
pub struct Resource<T> {
    value: T,
}
pub struct Resources {
    schemas: HashMap<String, Resource<Schema>>,
}
pub fn test() {
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
    print!("{}", light.debug_refs());
    let mss = format!("{light:#?}");
    print!("{}", mss);
}
