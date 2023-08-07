use super::{
    name_path::NamePath,
    plugin::{self, Plugin},
};
use crate::help::ecs::{Attach, Entity, GenericAttach};
use std::{
    cell::Cell,
    fmt::Debug,
    path::PathBuf,
    sync::{
        atomic::{self, AtomicUsize},
        Arc, RwLock, Weak,
    },
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
    fn get_mut(&mut self) -> &mut T {
        &mut self.val
    }
}
impl<T> Attach<Std> for Elem<T> {
    fn get(&self) -> &Std {
        &self.std
    }
    fn get_mut(&mut self) -> &mut Std {
        &mut self.std
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
    fn get_mut(&mut self) -> &mut T {
        &mut self.val
    }
}
impl<T> Attach<Std> for File<T> {
    fn get(&self) -> &Std {
        &self.std
    }
    fn get_mut(&mut self) -> &mut Std {
        &mut self.std
    }
}
impl<T> Attach<Dir> for File<T> {
    fn get(&self) -> &Dir {
        &self.dir
    }
    fn get_mut(&mut self) -> &mut Dir {
        &mut self.dir
    }
}
#[derive(Debug)]
pub struct Std {
    ///locally unique
    pub name: String,
    pub plugin: Weak<File<Plugin>>,
    pub completed: atomic::AtomicBool,
    pub id: AtomicUsize,
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
            id: AtomicUsize::default(),
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
    ///Whether the path is a relative path from the plugin folder path
    pub is_local: bool,
}
impl Dir {
    pub fn new(path: PathBuf, is_local: bool) -> Self {
        Dir { path, is_local }
    }
}
#[derive(Debug)]
pub struct InCode {
    pub name_path: NamePath,
}
impl InCode {
    pub fn new(name_path: NamePath) -> Self {
        InCode { name_path }
    }
}
pub trait Directory {
    fn abs_path(&self) -> PathBuf;
    fn local_path(&self) -> PathBuf;
}
impl<Entity> Directory for Entity
where
    Entity: Attach<Dir> + Attach<Std>,
{
    fn abs_path(&self) -> PathBuf {
        let dir = self.comp::<Dir>();
        if !dir.is_local {
            dir.path.clone()
        } else {
            let plugin_path = self.comp::<Std>().plugin.upgrade().unwrap().abs_path();
            plugin_path.join(dir.path.clone())
        }
    }
    fn local_path(&self) -> PathBuf {
        let dir = self.comp::<Dir>();
        if !dir.is_local {
            PathBuf::from("./")
        } else {
            dir.path.clone()
        }
    }
}
