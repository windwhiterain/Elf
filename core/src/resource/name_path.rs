pub trait NamePath {
    fn wrap(&mut self, name: String);
    fn name(&self) -> &String;
    fn plugin_name(&self) -> &String;
    fn prefixs(&self) -> Self;
    fn to_code(&self) -> String;
}
impl NamePath for Vec<String> {
    fn wrap(&mut self, name: String) {
        self.insert(0, name);
    }
    fn name(&self) -> &String {
        self.last().unwrap()
    }
    fn plugin_name(&self) -> &String {
        self.first().unwrap()
    }
    fn prefixs(&self) -> Self {
        (&self[0..self.len() - 1]).into()
    }
    fn to_code(&self) -> String {
        self.join(".")
    }
}
