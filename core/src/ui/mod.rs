pub mod resource_tree;
pub mod schema_tree;
///The backend independent description of an ui used for display and collect user input,config callback functions.
pub trait UIInfor<T> {
    ///Get the backend independent description of an ui
    fn gen_infor(&self) -> T;
}
