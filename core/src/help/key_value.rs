use std::default;

pub trait KeyValue<K, V> {
    fn key_value(&mut self, key: K, value: V);
}
impl<V> KeyValue<usize, V> for Vec<V>
where
    V: Default,
{
    fn key_value(&mut self, key: usize, value: V) {
        if self.len() <= key {
            self.resize_with(key + 1, || V::default())
        }
        self[key] = value;
    }
}
