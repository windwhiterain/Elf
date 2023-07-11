use crate::*;
///The only one context to manage resources in an elf applycation
pub struct Context {
    pub schemas: Resources<Schema>,
}
impl Context {
    pub fn new() -> Self {
        Context {
            schemas: Resources::new(),
        }
    }
}
///a wraper on raw rust struct that should be a resource load from plugin
pub struct Resource<T> {
    pub value: T,
    pub name: String,
}
impl<T> Resource<T> {
    pub fn new(name: String, value: T) -> Resource<T> {
        Resource { value, name }
    }
}
///Store a type of resources
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
    pub fn get(&self, id: usize) -> Option<Arc<Resource<T>>> {
        self.id_map.get(id)?.clone()
    }
}
