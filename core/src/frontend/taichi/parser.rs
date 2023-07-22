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

use super::help::*;
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

        for (name, bodies) in self.extract_schema(ast) {
            let schema = self.parse_schema(name, bodies, content, plugin.clone());
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
    ) -> Vec<(String, Vec<Located<StmtKind>>)> {
        let mut ret: Vec<(String, Vec<Located<StmtKind>>)> = vec![];
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
                        let d_name = get_name(&locate.node);
                        match d_name {
                            Some(v) => {
                                if v == self.schema_decorator_name {
                                    ret.push((name, body));
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
        bodies: Vec<Located<StmtKind>>,
        content: &mut resource::PluginsContent,
        plugin: Arc<Resource<Directory<Plugin>>>,
    ) -> Resource<Schema> {
        let mut struct_fields: Vec<(String, &schema::Schema)> = vec![];
        let mut prim_fields: Vec<(String, data::DataDescriptor)> = vec![];
        let mut calls: Vec<(String, Box<Located<ExprKind>>, Vec<Located<ExprKind>>)> = vec![];
        for locate in bodies {
            let stmt = locate.node;
            match stmt {
                StmtKind::AnnAssign {
                    target,
                    annotation,
                    value,
                    simple,
                } => {
                    let (field_name, value) = (get_name(&target.node).unwrap(), annotation.node);
                    match value {
                        ExprKind::Subscript { value, slice, ctx } => {
                            let (typename, dimension) = (value.node, slice.node);
                            prim_fields.push((
                                field_name,
                                DataDescriptor {
                                    dimension: get_int(dimension).unwrap(),
                                    typename: typename.into(),
                                },
                            ))
                        }
                        ExprKind::Call {
                            func,
                            args,
                            keywords,
                        } => {
                            calls.push((field_name, func, args));
                        }
                        expr => {
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
                _ => panic!(),
            }
        }
        let mut schema = Schema::new(struct_fields.into_iter(), prim_fields.into_iter());

        let mut shape_constraints: Vec<(String, Vec<Arc<data::ShapeConstraint>>, Vec<usize>)> =
            vec![];
        for call in calls {
            let (field_name, call_name, arg) = call;
            match call_name.node {
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
                                let temp = &schema.shape_constraint_maps[offset];
                                match schema.shape_constraint_maps[offset].get(name_path.name()) {
                                    Some(sc_ref) => {
                                        sc_refs.push(sc_ref.clone());
                                    }
                                    None => panic!(),
                                }
                            }
                        }
                    }
                    shape_constraints.push((field_name, sc_refs, prims));
                }
                _ => panic!(),
            }
        }
        schema.add_shape_constraints(shape_constraints.into_iter());
        Resource::new(name, schema, Some(plugin), true)
    }
}
