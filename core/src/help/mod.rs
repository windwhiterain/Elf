pub mod absolute_path;
pub mod ecs;
pub mod file;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
///Create a vec with the size of len filled with value
pub fn vec<T>(len: usize, value: T) -> Vec<T>
where
    T: Clone,
{
    let mut ret = Vec::<T>::with_capacity(len);
    ret.resize_with(len, || value.clone());
    ret
}
///Read only raw ptr of an arc usually used as key
pub trait ConstPtr<T> {
    fn get_const_ptr(&self) -> *const T;
}
impl<T> ConstPtr<T> for &T {
    fn get_const_ptr(&self) -> *const T {
        let ptr: *const T = *self;
        ptr
    }
}
///Copy an arc by clone the struct it pointing to
pub fn deep_copy<T>(arc: &Arc<T>) -> Arc<T>
where
    T: Clone,
{
    Arc::<T>::new((**arc).clone())
}
///A php flavour sytex to create a hashmap
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
struct CompresIter<'a, T, V: 'a>
where
    T: Iterator<Item = &'a Option<V>>,
{
    iter: T,
}
impl<'a, T, V> Iterator for CompresIter<'a, T, V>
where
    T: Iterator<Item = &'a Option<V>>,
{
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => {
                    return None;
                }
                Some(v) => match v {
                    None => {
                        continue;
                    }
                    Some(v) => {
                        return Some(v);
                    }
                },
            }
        }
    }
}
pub fn compress<'a, T: 'a>(
    iter: impl Iterator<Item = &'a Option<T>>,
) -> impl Iterator<Item = &'a T> {
    return CompresIter { iter };
}
