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
        self.names[0..self.names.len() - 1].iter()
    }
}
