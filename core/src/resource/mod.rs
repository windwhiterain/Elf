pub mod container;
pub mod name_path;
pub mod plugin;
use self::container::*;
use crate::{
    help::ecs::{Attach, Entity},
    *,
};
use help::*;
use name_path::NamePath;
use std::{
    cell::Cell,
    default, fs,
    marker::PhantomData,
    path::Path,
    sync::{atomic, Mutex, RwLock},
};

use self::plugin::ROOT_PLUGIN;
pub(crate) type Plugin = plugin::Plugin;
///The only one context to manage resources in an elf applycation
#[derive(Debug)]
pub struct Context {
    pub plugins: Resources<File<Plugin>>,
    pub plugins_content: PluginsContent,
    pub plugin_search_paths: Vec<PathBuf>,
}
#[derive(Debug, Default)]
pub struct PluginsContent {
    pub schemas: Resources<Elem<Schema>>,
}
impl PluginsContent {
    pub fn append(&mut self, mut other: PluginsContent) {
        self.schemas.append(other.schemas);
    }
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
                self.plugins.add(Arc::new(dir.path().clone().into()));
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
    pub fn append(&mut self, mut other: Self) {
        self.id_map.append(&mut other.id_map)
    }

    pub fn add(&mut self, resource: Arc<Entity>) {
        match self.try_get_vacancy() {
            Some(index) => self.id_map[index] = Some(resource),
            None => self.id_map.push(Some(resource)),
        };
    }
    pub fn get(&self, id: usize) -> Option<&Arc<Entity>> {
        self.id_map.get(id)?.as_ref()
    }
    pub fn find(
        &self,
        name_path: &NamePath,
        plugin: Option<&File<Plugin>>,
    ) -> Option<&Arc<Entity>> {
        let plugin = match plugin {
            Some(v) => v,
            None => &ROOT_PLUGIN,
        };
        for value in &self.id_map {
            if let Some(resource) = value.as_ref() {
                let resource_plugin = &resource.get().plugin.upgrade().unwrap();
                if name_path.name() == &resource.get().name {
                    if resource_plugin.get_const_ptr() == plugin.get_const_ptr()
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
}

#[test]
fn test() {
    let mut context = starter::test_initialize();
    context.resource.load_plugins();
    let t = context.resource.plugins_content.schemas.get(3).unwrap();
    t.display(&context);
}
