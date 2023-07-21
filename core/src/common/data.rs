use crate::help::vec;
use crate::resource::NamePath;
#[derive(Debug)]
pub struct Data {}
#[derive(Debug)]
pub struct DataDescriptor {
    pub dimension: usize,
    pub typename: NamePath,
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
