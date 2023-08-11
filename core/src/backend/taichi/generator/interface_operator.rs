use crate::{
    graph::operator::{DataFrom, InterfaceOperator},
    resource::name_path::NamePath,
};

use super::{code_line::CodeLines, flaten_data_name, flaten_interface_name};
pub fn to_code(interface_operator: &InterfaceOperator) -> CodeLines {
    let mut code = CodeLines::new();
    let interface = flaten_interface_name(interface_operator.interface);
    for link in &interface_operator.links {
        let to_path = link.to.to_code();
        let (from_path, is_end) = match &link.from {
            DataFrom::Data(data) => (flaten_data_name(*data), "True"),
            DataFrom::Interface { name, prim } => {
                let path = prim.name_path.to_code();
                (format!("{name}.{path}"), "False")
            }
        };
        code.write(
            0,
            format!(
                "context.{interface}.{to_path}=ChainRef(is_end={is_end},value=context.{from_path})"
            ),
        );
    }
    code
}
