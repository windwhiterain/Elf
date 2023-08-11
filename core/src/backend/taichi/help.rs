use rustpython_ast::{Constant, ExprKind};

use crate::resource::name_path::NamePath;

pub fn get_name(expr: &ExprKind) -> Option<String> {
    match expr {
        ExprKind::Name { id, ctx } => Some(id.clone()),
        _ => None,
    }
}
pub fn get_int(node: ExprKind) -> Option<usize> {
    match node {
        ExprKind::Constant { value, kind } => match value {
            Constant::Int(int) => {
                let (sign, vec) = int.to_u32_digits();
                match sign {
                    num_bigint::Sign::Minus => panic!(),
                    _ => (),
                }
                if vec.len() == 0 {
                    Some(0)
                } else {
                    Some(vec[0].try_into().unwrap())
                }
            }
            _ => None,
        },
        _ => None,
    }
}
pub fn get_name_path(value: &ExprKind) -> Vec<String> {
    let mut names = vec![];
    let mut current = value;
    loop {
        match current {
            ExprKind::Name { id, ctx } => {
                names.push(id.clone());
                break;
            }
            ExprKind::Attribute { value, attr, ctx } => {
                names.push(attr.clone());
                current = &value.node;
            }
            _ => panic!(),
        }
    }
    names.reverse();
    names.into()
}
