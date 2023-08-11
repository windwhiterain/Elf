use std::{
    fs,
    path::{Path, PathBuf},
};

pub trait AbsolutePath {
    fn abs_path(&self) -> PathBuf;
}
impl<T> AbsolutePath for T
where
    T: AsRef<Path>,
{
    fn abs_path(&self) -> PathBuf {
        fs::canonicalize(self).unwrap()
    }
}
