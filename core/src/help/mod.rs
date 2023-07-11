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
///Whether two arc are pointing to a same struct
pub fn eq<T>(left: &Arc<T>, right: &Arc<T>) -> bool {
    ptr(left) == ptr(right)
}
///Read only raw ptr of an arc usually used as key
pub fn ptr<T>(arc: &Arc<T>) -> *const T {
    let ptr: *const T = arc.as_ref();
    ptr
}
///Copy an arc by clone the struct it pointing to
pub fn deep_copy<T>(arc: &Arc<T>) -> Arc<T>
where
    T: Clone,
{
    Arc::<T>::new((**arc).clone())
}
