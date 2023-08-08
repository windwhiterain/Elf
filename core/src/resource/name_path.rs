#[derive(Debug)]
pub struct NamePath {
    names: Vec<String>,
}
impl From<Vec<String>> for NamePath {
    fn from(names: Vec<String>) -> Self {
        NamePath { names }
    }
}
impl From<Vec<&str>> for NamePath {
    fn from(names: Vec<&str>) -> Self {
        NamePath {
            names: Vec::from_iter(names.iter().map(|a| a.to_string())),
        }
    }
}
impl From<String> for NamePath {
    fn from(name: String) -> Self {
        NamePath { names: vec![name] }
    }
}
impl From<&str> for NamePath {
    fn from(value: &str) -> Self {
        NamePath {
            names: vec![value.to_string()],
        }
    }
}
impl From<&[String]> for NamePath {
    fn from(value: &[String]) -> Self {
        NamePath {
            names: Vec::from_iter(value.into_iter().map(|a| a.clone())),
        }
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
    pub fn prefixs(&self) -> NamePath {
        (&self.names[0..self.names.len() - 1]).into()
    }
    pub fn all(&self) -> impl Iterator<Item = &String> {
        self.names.iter()
    }
}
