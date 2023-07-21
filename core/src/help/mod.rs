use std::sync::Arc;
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
impl<T> ConstPtr<T> for &Arc<T> {
    fn get_const_ptr(&self) -> *const T {
        self.get_const_ptr()
    }
}
///Copy an arc by clone the struct it pointing to
pub fn deep_copy<T>(arc: &Arc<T>) -> Arc<T>
where
    T: Clone,
{
    Arc::<T>::new((**arc).clone())
}
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
