use crate::{help::ecs::Attach, resource::container::File};

use super::Input;

pub type DataOperatorR = File<DataOperator>;
#[derive(Debug)]
pub struct DataOperator {
    input: Input,
}
impl DataOperator {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}
impl Attach<Input> for DataOperator {
    fn get(&self) -> &Input {
        &self.input
    }
    fn get_mut(&mut self) -> &mut Input {
        &mut self.input
    }
}
