mod plugin;
use std::{
    cell::Cell,
    default, fs,
    path::Path,
    sync::{atomic, Mutex, RwLock},
};

use crate::*;
use help::*;

use self::plugin::ROOT_PLUGIN;
pub(crate) type Plugin = plugin::Plugin;
///The only one context to manage resources in an elf applycation
#[derive(Debug)]
pub struct Context {
    pub plugins: Resources<Directory<Plugin>>,
    pub plugins_content: PluginsContent,
    pub plugin_search_paths: Vec<PathBuf>,
}
#[derive(Debug, Default)]
pub struct PluginsContent {
    pub schemas: Resources<Schema>,
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
///a wraper on raw rust struct that should be a resource load from plugin
#[derive(Debug)]
pub struct Resource<T> {
    pub value: T,
    pub name: String,
    pub plugin: Weak<Resource<Directory<Plugin>>>,
    pub completed: atomic::AtomicBool,
}
impl<T> Resource<T> {
    pub fn new(
        name: String,
        value: T,
        plugin: Option<Arc<Resource<Directory<Plugin>>>>,
        completed: bool,
    ) -> Resource<T> {
        Resource {
            value,
            name,
            plugin: match plugin {
                Some(v) => Arc::downgrade(&v),
                None => Arc::downgrade(&plugin::ROOT_PLUGIN),
            },
            completed: completed.into(),
        }
    }
    pub fn try_complete(&self) -> bool {
        !self
            .completed
            .swap(true, std::sync::atomic::Ordering::AcqRel)
    }
}
///Store a type of resources
#[derive(Debug)]
pub struct Resources<T> {
    pub id_map: Vec<Option<Arc<Resource<T>>>>,
}
impl<T> Default for Resources<T> {
    fn default() -> Self {
        Resources {
            id_map: Vec::default(),
        }
    }
}
impl<T> Resources<T> {
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
    pub fn append(&mut self, mut other: Resources<T>) {
        self.id_map.append(&mut other.id_map)
    }

    pub fn add(&mut self, resource: Arc<Resource<T>>) {
        match self.try_get_vacancy() {
            Some(index) => self.id_map[index] = Some(resource),
            None => self.id_map.push(Some(resource)),
        };
    }
    pub fn get(&self, id: usize) -> Option<&Arc<Resource<T>>> {
        self.id_map.get(id)?.as_ref()
    }
    pub fn find(
        &self,
        name_path: &NamePath,
        plugin: Option<&Resource<Directory<Plugin>>>,
    ) -> Option<&Arc<Resource<T>>> {
        let plugin = match plugin {
            Some(v) => v,
            None => &ROOT_PLUGIN,
        };
        for value in &self.id_map {
            if let Some(resource) = value.as_ref() {
                let resource_plugin = &resource.plugin.upgrade().unwrap();
                if name_path.name() == &resource.name {
                    if resource_plugin.get_const_ptr() == plugin.get_const_ptr()
                        || &resource_plugin.name == name_path.plugin_name()
                    {
                        return Some(&resource);
                    }
                }
            }
        }
        None
    }
    pub fn get_all(&self) -> impl Iterator<Item = &Arc<Resource<T>>> {
        compress(self.id_map.iter())
    }
}
#[derive(Debug)]
pub struct Directory<T> {
    pub value: T,
    pub path: PathBuf,
}
impl<T> Directory<T> {
    pub fn new(value: T, path: PathBuf) -> Self {
        Directory { value, path }
    }
}

#[derive(Debug)]
pub struct NamePath {
    names: Vec<String>,
}
impl From<Vec<String>> for NamePath {
    fn from(names: Vec<String>) -> Self {
        NamePath { names }
    }
}
impl From<String> for NamePath {
    fn from(name: String) -> Self {
        NamePath { names: vec![name] }
    }
}
impl NamePath {
    pub fn wrap(&mut self, name: String) {
        self.names.insert(0, name);
    }
    pub fn name(&self) -> &String {
        self.names.last().unwrap()
    }
    pub fn plugin_name(&self) -> &String {
        self.names.first().unwrap()
    }
    pub fn prefixs(&self) -> impl Iterator<Item = &String> {
        self.names[0..self.names.len() - 2].iter()
    }
}
#[test]
fn test() {
    let mut context = starter::test_initialize();
    context.resource.load_plugins();
    context
        .resource
        .plugins_content
        .schemas
        .get(1)
        .unwrap()
        .display(&context);
}
