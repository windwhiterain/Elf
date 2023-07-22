mod help;
mod parser;
use crate::resource::*;
use rustpython_ast::*;
pub type Parser = parser::Parser;
impl From<ExprKind> for NamePath {
    fn from(value: ExprKind) -> Self {
        let mut names = vec![];
        let mut current = value;
        loop {
            match current {
                ExprKind::Name { id, ctx } => {
                    names.push(id);
                    break;
                }
                ExprKind::Attribute { value, attr, ctx } => {
                    names.push(attr);
                    current = value.node;
                }
                _ => panic!(),
            }
        }
        names.reverse();
        names.into()
    }
}
