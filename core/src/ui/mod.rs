pub mod schema_tree;
use pyo3::prelude::*;
pub fn gen_module(py: Python, m: &PyModule) -> PyResult<()> {
    let sub_m = PyModule::new(py, "ui")?;
    schema_tree::gen_module(py, sub_m)?;
    m.add_submodule(sub_m)?;
    Ok(())
}
///The backend independent description of an ui used for display and collect user input
pub trait UIInfor<T> {
    ///Get the backend independent description of an ui
    fn gen_infor(&self) -> T;
}
