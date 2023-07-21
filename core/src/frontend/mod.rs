use std::{path::PathBuf, sync::Arc};

use crate::{
    common::Schema,
    resource::{Plugin, Resource},
    Context,
};

mod taichi;
pub trait Parser {
    fn parse(&self, context: &mut crate::Context, plugin: &Arc<Resource<Plugin>>, folder: PathBuf);
}
