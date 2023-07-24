use super::plugin::{self, Plugin};
use crate::help::ecs::{Attach, GenericAttach};
use std::{
    fmt::Debug,
    path::PathBuf,
    sync::{atomic, Arc, Weak},
};
#[derive(Debug)]
pub struct Elem<T> {
    pub val: T,
    pub std: Std,
}
impl<T> GenericAttach<T> for Elem<T> {
    fn get(&self) -> &T {
        &self.val
    }
}
impl<T> Attach<Std> for Elem<T> {
    fn get(&self) -> &Std {
        &self.std
    }
}
#[derive(Debug)]
pub struct File<T> {
    pub val: T,
    pub std: Std,
    pub dir: Dir,
}
impl<T> GenericAttach<T> for File<T> {
    fn get(&self) -> &T {
        &self.val
    }
}
impl<T> Attach<Std> for File<T> {
    fn get(&self) -> &Std {
        &self.std
    }
}
impl<T> Attach<Dir> for File<T> {
    fn get(&self) -> &Dir {
        &self.dir
    }
}
#[derive(Debug)]
pub struct Std {
    ///locally unique
    pub name: String,
    pub plugin: Weak<File<Plugin>>,
    pub completed: atomic::AtomicBool,
}
impl Std {
    pub fn new(name: String, plugin: Option<Arc<File<Plugin>>>, completed: bool) -> Std {
        Std {
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
#[derive(Debug)]
pub struct Dir {
    pub path: PathBuf,
}
impl Dir {
    pub fn new(path: PathBuf) -> Self {
        Dir { path }
    }
}
