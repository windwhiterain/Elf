use std::iter;
use std::path::PathBuf;
use std::sync::Arc;
use std::usize;

use crate::common::structure::PrimField;
use crate::common::structure::StructAccess;
use crate::common::*;
use crate::frontend;
use crate::resource;
use crate::resource::Directory;
use crate::resource::NamePath;
use crate::resource::Plugin;
use crate::resource::Resource;
use crate::resource::Resources;
use rustpython_ast::*;
use rustpython_parser::parser;
pub struct Parser {
    schema_decorator_name: String,
    shape_constraint_name: String,
}
impl frontend::Parser for Parser {
    fn parse_code(
        &self,
        content: &mut resource::PluginsContent,
        plugin: &Arc<Resource<Directory<Plugin>>>,
        code: String,
    ) {
        let ast = parser::parse_program(&code, "").unwrap();

        for (name, kw) in self.extract_schema(ast) {
            let schema = self.parse_schema(name, kw, content, plugin.clone());
            content.schemas.add(Arc::new(schema));
        }
    }
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            schema_decorator_name: "schema".into(),
            shape_constraint_name: "shape_constraint".into(),
        }
    }
    pub fn extract_schema(
        &self,
        stmts: Vec<Located<StmtKind>>,
    ) -> Vec<(String, Vec<Located<KeywordData>>)> {
        let mut ret = vec![];
        for locate in stmts {
            let node = locate.node;
            match node {
                StmtKind::ClassDef {
                    name,
                    bases,
                    keywords,
                    body,
                    decorator_list,
                } => {
                    for locate in decorator_list {
                        let node = locate.node;
                        match node {
                            ExprKind::Name { id, ctx } => {
                                if id == self.schema_decorator_name {
                                    ret.push((name, keywords));
                                    break;
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        ret
    }
    pub fn parse_schema(
        &self,
        name: String,
        decorator_list: Vec<Located<KeywordData>>,
        content: &mut resource::PluginsContent,
        plugin: Arc<Resource<Directory<Plugin>>>,
    ) -> Resource<Schema> {
        let mut struct_fields = vec![];
        let mut prim_fields = vec![];
        let mut calls = vec![];
        for locate in decorator_list {
            let kw = locate.node;
            let (field_name, value) = (kw.arg.unwrap(), kw.value.node);
            match value {
                ExprKind::Subscript { value, slice, ctx } => {
                    let (typename, dimension) = (value.node, slice.node);
                    prim_fields.push((
                        field_name,
                        DataDescriptor {
                            dimension: try_to_int(dimension).unwrap(),
                            typename: typename.into(),
                        },
                    ))
                }
                ExprKind::Call {
                    func,
                    args,
                    keywords,
                } => {
                    calls.push((func, args));
                }
                (expr) => {
                    let name_path = expr.into();
                    let schema = &content
                        .schemas
                        .find(&name_path, Some(&plugin))
                        .unwrap()
                        .value;
                    struct_fields.push((field_name, schema))
                }
            }
        }
        let mut schema = Schema::new(struct_fields.into_iter(), prim_fields.into_iter());

        let mut shape_constraints = vec![];
        for call in calls {
            let (name, arg) = call;
            match name.node {
                ExprKind::Name { id, ctx } => {
                    assert!(&id == &self.shape_constraint_name);
                    let mut sc_refs = vec![];
                    let mut prims = vec![];
                    for locate in arg {
                        let name_path = NamePath::from(locate.node);
                        let access = StructAccess::from(&schema.structure);
                        let target = access.find_struct(&name_path).unwrap();
                        match target.find_prim_offset(name_path.name()) {
                            Some(offset) => prims.push(offset),
                            None => {
                                let offset = target.get_struct_offset();
                                match schema.shape_constraint_maps[offset].get(name_path.name()) {
                                    Some(sc_ref) => {
                                        sc_refs.push(sc_ref.clone());
                                    }
                                    None => panic!(),
                                }
                            }
                        }
                    }
                    shape_constraints.push((id, sc_refs, prims));
                }
                _ => panic!(),
            }
        }
        schema.add_shape_constraints(shape_constraints.into_iter());
        Resource::new(name, schema, Some(plugin), true)
    }
}
fn try_to_int(node: ExprKind) -> Option<usize> {
    match node {
        ExprKind::Constant { value, kind } => match value {
            Constant::Int(int) => {
                let (sign, vec) = int.to_u32_digits();
                match sign {
                    num_bigint::Sign::Minus => panic!(),
                    _ => (),
                }
                Some(vec[0].try_into().unwrap())
            }
            _ => None,
        },
        _ => None,
    }
}
