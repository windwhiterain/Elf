use std::sync::Arc;

pub fn vec<T>(len: usize, value: T) -> Vec<T>
where
    T: Clone,
{
    let mut ret = Vec::<T>::with_capacity(len);
    ret.resize_with(len, || value.clone());
    ret
}
pub fn eq<T>(left: &Arc<T>, right: &Arc<T>) -> bool {
    ptr(left) == ptr(right)
}
pub fn ptr<T>(arc: &Arc<T>) -> *const T {
    let ptr: *const T = arc.as_ref();
    ptr
}
pub fn deep_copy<T>(arc: &Arc<T>) -> Arc<T>
where
    T: Clone,
{
    Arc::<T>::new((**arc).clone())
}
