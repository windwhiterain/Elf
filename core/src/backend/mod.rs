use std::{path::PathBuf, sync::Arc};

use crate::{
    common::{schema::SchemaR, Schema},
    graph::Graph,
    resource::{self, container::File, plugin::PluginR, Plugin, PluginsContent, Resources},
    Context,
};

pub mod taichi;
pub struct CodeFile {
    pub local_path: PathBuf,
    pub code: String,
}
pub trait Parser {
    fn parse_codes(
        &self,
        content: &mut resource::PluginsContent,
        plugin: &Arc<PluginR>,
        codes: Vec<CodeFile>,
    ) {
        for code in codes {
            self.parse_code(content, plugin, code)
        }
    }
    fn parse_code(
        &self,
        content: &mut resource::PluginsContent,
        plugin: &Arc<PluginR>,
        code: CodeFile,
    );
}
pub trait Generator<'a> {
    fn generate(&self, context: &Context);
}
