use std::{path::PathBuf, sync::Arc};

use crate::{
    common::Schema,
    resource::{self, Directory, Plugin, Resource, Resources},
    Context,
};

pub mod taichi;
pub trait Parser {
    fn parse_codes(
        &self,
        content: &mut resource::PluginsContent,
        plugin: &Arc<Resource<Directory<Plugin>>>,
        codes: Vec<String>,
    ) {
        for code in codes {
            self.parse_code(content, plugin, code)
        }
    }
    fn parse_code(
        &self,
        content: &mut resource::PluginsContent,
        plugin: &Arc<Resource<Directory<Plugin>>>,
        code: String,
    );
}
