use std::{collections::HashMap, sync::Weak};

use super::schema::SchemaR;

pub mod data_operator;
#[derive(Debug)]
pub struct Input {
    pub schemas: HashMap<String, Weak<SchemaR>>,
}

impl Input {
    pub fn new(schemas: HashMap<String, Weak<SchemaR>>) -> Self {
        Self { schemas }
    }
}
