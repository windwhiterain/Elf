use crate::help::vec;
use crate::resource::name_path::NamePath;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
}
#[derive(Debug)]
pub struct Descriptor {
    pub dimension: usize,
    pub data_type: Type,
}
#[derive(Debug, Clone)]
pub struct ShapeConstraint {
    pub shape: Vec<i32>,
}
impl ShapeConstraint {
    pub fn new(dimension: usize) -> ShapeConstraint {
        ShapeConstraint {
            shape: vec(dimension, 0),
        }
    }
    pub fn dimension(&self) -> usize {
        self.shape.len()
    }
}
