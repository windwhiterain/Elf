use std::{fs, path::PathBuf};

pub struct Node {
    path: PathBuf,
}
impl From<PathBuf> for Node {
    fn from(path: PathBuf) -> Self {
        Node { path: path }
    }
}

impl Node {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_string_lossy().to_string()
    }
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }
    pub fn get_local_dirs(&self) -> impl Iterator<Item = Node> {
        let mut ret: Vec<Node> = vec![];
        for dir in fs::read_dir(&self.path).unwrap() {
            let entry = dir.unwrap();
            if !entry.file_type().unwrap().is_dir() {
                continue;
            }
            ret.push(entry.path().into())
        }
        ret.into_iter()
    }
    pub fn get_local_files(&self, suffix: &str) -> impl Iterator<Item = Node> {
        let mut ret: Vec<Node> = vec![];
        for dir in fs::read_dir(&self.path).unwrap() {
            let entry = dir.unwrap();
            if entry.file_type().unwrap().is_dir()
                || !entry.file_name().to_str().unwrap().ends_with(suffix)
            {
                continue;
            }
            ret.push(entry.path().into())
        }
        ret.into_iter()
    }
    pub fn read(&self) -> String {
        fs::read_to_string(&self.path).unwrap()
    }
    pub fn get_files(&self, suffix: &str) -> Vec<Node> {
        let mut ret = Vec::<Node>::from_iter(self.get_local_files(suffix));
        for node in self.get_local_dirs() {
            ret.append(&mut node.get_files(suffix));
        }
        ret
    }
}
