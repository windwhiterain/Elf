pub mod interface_operator;
use pyo3::{pyclass, pymethods};
#[derive(Debug, Clone)]
#[pyclass]
pub struct Network {
    #[pyo3(get, set)]
    pub nodes: Vec<Node>,
    #[pyo3(get, set)]
    pub interface_operators: Vec<interface_operator::InterfaceOperator>,
    #[pyo3(get, set)]
    pub input_datas: Vec<InputData>,
}
#[pymethods]
impl Network {
    #[new]
    pub fn new() -> Network {
        Network {
            nodes: vec![],
            interface_operators: vec![],
            input_datas: vec![],
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass]
pub struct Node {
    #[pyo3(get, set)]
    pub input: Vec<usize>,
    #[pyo3(get, set)]
    pub node_type: NodeType,
    #[pyo3(get, set)]
    pub type_index: usize,
    #[pyo3(get, set)]
    pub pos: [f32; 2],
}
#[pymethods]
impl Node {
    #[new]
    pub fn new(node_type: NodeType, type_index: usize, pos: [f32; 2]) -> Node {
        Node {
            input: vec![],
            node_type,
            type_index,
            pos,
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass]
pub enum NodeType {
    DataOperator,
    InterfaceOperator,
    InputData,
}
#[derive(Debug, Clone)]
#[pyclass]
pub struct InputData {
    #[pyo3(get, set)]
    pub schema: usize,
}
#[pymethods]
impl InputData {
    #[new]
    pub fn new(schema: usize) -> InputData {
        InputData { schema }
    }
}
