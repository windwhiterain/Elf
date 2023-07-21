mod plugin;
use std::{default, fs};

use crate::*;
use help::*;
pub(crate) type Plugin = plugin::Plugin;
///The only one context to manage resources in an elf applycation
#[derive(Debug)]
pub struct Context {
    pub plugins: Resources<Directory<Plugin>>,
    pub schemas: Resources<Schema>,
    pub plugin_search_paths: Vec<PathBuf>,
}
impl Context {
    pub fn new() -> Self {
        Context {
            plugins: Resources::new(),
            schemas: Resources::new(),
            plugin_search_paths: vec![PathBuf::from("../plugin")],
        }
    }
    fn scan_plugin(&mut self) {
        for search_path in &self.plugin_search_paths {
            for dir in fs::read_dir(search_path).unwrap() {
                let entry = dir.unwrap();
                if !entry.file_type().unwrap().is_dir() {
                    continue;
                }
                let (_, path) = (entry.file_name(), entry.path());
                self.plugins.add(Arc::new(path.into()));
            }
        }
    }
}
///a wraper on raw rust struct that should be a resource load from plugin
#[derive(Debug)]
pub struct Resource<T> {
    pub value: T,
    pub name: String,
    pub plugin: Weak<Resource<Plugin>>,
}
impl<T> Resource<T> {
    pub fn new(name: String, value: T, plugin: Arc<Resource<Plugin>>) -> Resource<T> {
        Resource {
            value,
            name,
            plugin: Arc::downgrade(&plugin),
        }
    }
}
///Store a type of resources
#[derive(Debug)]
pub struct Resources<T> {
    pub id_map: Vec<Option<Arc<Resource<T>>>>,
}
impl<T> Resources<T> {
    pub fn new() -> Resources<T> {
        {
            Resources { id_map: Vec::new() }
        }
    }
    pub fn add(&mut self, resource: Arc<Resource<T>>) -> usize {
        for (index, value) in self.id_map.iter().enumerate() {
            match value {
                None => {
                    self.id_map[index] = Some(resource);
                    return index;
                }
                Some(_) => {}
            }
        }
        self.id_map.push(Some(resource));
        self.id_map.len() - 1
    }
    pub fn get(&self, id: usize) -> Option<&Arc<Resource<T>>> {
        self.id_map.get(id)?.as_ref()
    }
    pub fn find(
        &self,
        name_path: NamePath,
        plugin: &Resource<Plugin>,
    ) -> Option<&Arc<Resource<T>>> {
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
}
#[derive(Debug)]
pub struct Directory<T> {
    pub value: T,
    pub path: PathBuf,
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
    context.resource.scan_plugin();
    println!("{:#?}", context.resource.plugins);
}
