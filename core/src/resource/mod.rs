pub mod container;
pub mod name_path;
pub mod plugin;
use self::{container::*, plugin::PluginR};
use crate::{
    common::{
        operator::data_operator::{DataOperator, DataOperatorR},
        schema::SchemaR,
    },
    help::ecs::{Attach, Entity},
    *,
};
use help::*;
use name_path::NamePath;
use std::{
    cell::Cell,
    default, fs,
    marker::PhantomData,
    ops::Deref,
    path::Path,
    sync::{atomic, Mutex, RwLock},
};

use self::plugin::ROOT_PLUGIN;
pub(crate) type Plugin = plugin::Plugin;
///The only one context to manage resources in an elf applycation
#[derive(Debug)]
pub struct Context {
    pub plugins: Resources<PluginR>,
    pub plugins_content: PluginsContent,
    pub plugin_search_paths: Vec<PathBuf>,
}
#[derive(Debug, Default)]
pub struct PluginsContent {
    pub schemas: Resources<SchemaR>,
    pub data_operators: Resources<DataOperatorR>,
}
impl Context {
    pub fn new() -> Self {
        Context {
            plugins: Resources::default(),
            plugins_content: PluginsContent::default(),
            plugin_search_paths: vec![PathBuf::from("../plugin")],
        }
    }
    fn scan_plugins(&mut self) {
        for search_path in &self.plugin_search_paths {
            let node = file::Node::from(search_path.clone());
            for dir in node.get_all_dir() {
                let path = dir.path().clone();
                self.plugins.add(Arc::new(path.into()));
            }
        }
    }
    fn completed_plugins(&mut self) {
        for plugin in self.plugins.get_all() {
            plugin::complete(plugin, &self.plugins, &mut self.plugins_content);
        }
    }
    pub fn load_plugins(&mut self) {
        self.scan_plugins();
        self.completed_plugins();
    }
}
///Store a type of resources
#[derive(Debug)]
pub struct Resources<Entity> {
    pub id_map: Vec<Option<Arc<Entity>>>,
}
impl<Entity> Default for Resources<Entity> {
    fn default() -> Self {
        Resources {
            id_map: Vec::default(),
        }
    }
}
impl<Entity> Resources<Entity>
where
    Entity: Attach<Std>,
{
    fn try_get_vacancy(&self) -> Option<usize> {
        for (index, value) in self.id_map.iter().enumerate() {
            match value {
                None => {
                    return Some(index);
                }
                Some(_) => {}
            }
        }
        None
    }

    pub fn add(&mut self, resource: Arc<Entity>) {
        match self.try_get_vacancy() {
            Some(index) => {
                resource
                    .as_ref()
                    .comp::<Std>()
                    .id
                    .store(index, atomic::Ordering::Relaxed);
                self.id_map[index] = Some(resource)
            }
            None => {
                resource
                    .as_ref()
                    .comp::<Std>()
                    .id
                    .store(self.id_map.len(), atomic::Ordering::Relaxed);
                self.id_map.push(Some(resource))
            }
        };
    }
    pub fn get(&self, id: usize) -> Option<&Arc<Entity>> {
        self.id_map.get(id)?.as_ref()
    }
    pub fn find(
        &self,
        name_path: &NamePath,
        environment: Option<&PluginR>,
    ) -> Option<&Arc<Entity>> {
        let plugin = match environment {
            Some(v) => v,
            None => &ROOT_PLUGIN,
        };
        for value in &self.id_map {
            if let Some(resource) = value.as_ref() {
                let resource_plugin = &resource.get().plugin.upgrade().unwrap();
                if name_path.name() == &resource.get().name {
                    if resource_plugin.as_ref().get_const_ptr() == plugin.get_const_ptr()
                        || &resource_plugin.as_ref().comp::<Std>().name == name_path.plugin_name()
                    {
                        return Some(&resource);
                    }
                }
            }
        }
        None
    }
    pub fn get_all(&self) -> impl Iterator<Item = &Arc<Entity>> {
        compress(self.id_map.iter())
    }
    pub fn filter_by_plugins<'a>(
        &'a self,
        plugins: &'a Vec<&PluginR>,
    ) -> impl Iterator<Item = &Arc<Entity>> + 'a {
        self.get_all().filter(move |r| {
            for plugin in plugins {
                if r.as_ref()
                    .get()
                    .plugin
                    .upgrade()
                    .unwrap()
                    .as_ref()
                    .get_const_ptr()
                    == plugin.get_const_ptr()
                {
                    return true;
                }
            }
            false
        })
    }
}

#[test]
fn test() {
    let mut context = starter::test_initialize();
    context.resource.load_plugins();
    let t = context.resource.plugins_content.schemas.get(1).unwrap();
    t.display(&context);
}
