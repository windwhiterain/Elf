mod common;
use std::{collections::HashMap, sync::Arc};

use common::*;
pub struct Context {}
pub struct Resource<T> {
    value: T,
}
pub struct Resources {
    schemas: HashMap<String, Resource<Schema>>,
}
