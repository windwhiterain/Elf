use pyo3::{pyclass, pymethods};

#[derive(Debug, Default, Clone)]
#[pyclass]
pub struct InterfaceOperator {
    #[pyo3(get, set)]
    pub links: Vec<Link>,
}
#[pymethods]
impl InterfaceOperator {
    #[new]
    pub fn new() -> InterfaceOperator {
        InterfaceOperator { links: vec![] }
    }
}
#[derive(Debug, Default, Clone)]
#[pyclass]
pub struct Link {
    #[pyo3(get, set)]
    pub input_port: usize,
    #[pyo3(get, set)]
    pub output_port: usize,
    #[pyo3(get, set)]
    pub input_path: Vec<String>,
    #[pyo3(get, set)]
    pub output_path: Vec<String>,
}
#[pymethods]
impl Link {
    #[new]
    pub fn new(input_port: usize, output_port: usize) -> Link {
        Link {
            input_port,
            output_port,
            input_path: vec![],
            output_path: vec![],
        }
    }
}
