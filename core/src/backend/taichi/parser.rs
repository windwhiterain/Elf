use std::collections::HashMap;
use std::iter;
use std::iter::Map;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use std::usize;

use crate::backend;
use crate::backend::CodeFile;
use crate::common;
use crate::common::operator::data_operator::DataOperator;
use crate::common::operator::data_operator::DataOperatorR;
use crate::common::schema::SchemaR;
use crate::common::structure::PrimField;
use crate::common::structure::StructAccess;
use crate::common::*;
use crate::hashmap;
use crate::help::ecs::Entity;
use crate::resource;
use crate::resource::container::Elem;
use crate::resource::container::File;
use crate::resource::container::Std;
use crate::resource::name_path;
use crate::resource::name_path::NamePath;
use crate::resource::plugin::PluginR;
use crate::resource::Plugin;
use crate::resource::Resources;
use rustpython_ast::*;
use rustpython_parser::parser;

use super::help::*;
struct Nodes<T> {
    value: Vec<Located<T>>,
}
impl<T> Nodes<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.value.iter().map(|l| &l.node)
    }
}
impl<T> From<Vec<Located<T>>> for Nodes<T> {
    fn from(value: Vec<Located<T>>) -> Self {
        Nodes::<T> { value }
    }
}
type Stmts = Nodes<StmtKind>;
type Exprs = Nodes<ExprKind>;
struct ClassDef {
    pub name: String,
    pub bases: Exprs,
    pub body: Stmts,
}

pub struct Parser {
    schema_decorator_name: &'static str,
    data_operator_decorator_name: &'static str,
    shape_constraint_name: &'static str,
}
impl backend::Parser for Parser {
    fn parse_code(
        &self,
        content: &mut resource::PluginsContent,
        plugin: &Arc<File<Plugin>>,
        code_file: CodeFile,
    ) {
        let ast: Stmts = parser::parse_program(&code_file.code, "").unwrap().into();

        for class in self.extract_class(&ast, self.schema_decorator_name) {
            let schema = self.parse_schema(class, content, plugin.clone());
            content.schemas.add(Arc::new(schema));
        }
        for class in self.extract_class(&ast, self.data_operator_decorator_name) {
            let data_operator = self.parse_data_operator(
                class,
                content,
                plugin.clone(),
                code_file.local_path.clone(),
            );
            content.data_operators.add(Arc::new(data_operator));
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            schema_decorator_name: "schema",
            data_operator_decorator_name: "data_operator",
            shape_constraint_name: "shape_constraint",
        }
    }
    fn extract_class(&self, body: &Stmts, decorator_name: &str) -> Vec<ClassDef> {
        let mut ret: Vec<ClassDef> = vec![];
        for stmt in body.iter() {
            match stmt {
                StmtKind::ClassDef {
                    name,
                    bases,
                    keywords,
                    body,
                    decorator_list,
                } => {
                    let decorators: Exprs = decorator_list.clone().into();
                    for expr in decorators.iter() {
                        let d_name = get_name(expr);
                        match d_name {
                            Some(v) => {
                                if v == decorator_name {
                                    ret.push(ClassDef {
                                        name: name.clone(),
                                        bases: bases.clone().into(),
                                        body: body.clone().into(),
                                    });
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
    fn parse_schema(
        &self,
        class: ClassDef,
        content: &resource::PluginsContent,
        plugin: Arc<PluginR>,
    ) -> SchemaR {
        let mut struct_fields: Vec<(String, &schema::Schema)> = vec![];
        let mut prim_fields: Vec<(String, data::Descriptor)> = vec![];
        let mut calls: Vec<(String, Box<Located<ExprKind>>, Vec<Located<ExprKind>>)> = vec![];
        for stmt in class.body.iter() {
            match stmt.clone() {
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
                                    data_type: super::DATA_TYPE_MAP
                                        .deref()
                                        .get_by_left(get_name(&typename).unwrap().as_str())
                                        .unwrap()
                                        .clone(),
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
                            let name_path = get_name_path(&expr);
                            let schema = &content
                                .schemas
                                .find(&name_path, Some(&plugin))
                                .unwrap()
                                .as_ref()
                                .val;
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
                        let name_path = get_name_path(&locate.node);
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
        Elem {
            val: schema,
            std: Std::new(class.name, Some(plugin), true),
        }
    }
    fn parse_data_operator(
        &self,
        class: ClassDef,
        content: &mut resource::PluginsContent,
        plugin: Arc<PluginR>,
        local_path: PathBuf,
    ) -> DataOperatorR {
        let mut input = crate::hashmap! {};
        for stmt in class.body.iter() {
            match stmt {
                StmtKind::FunctionDef {
                    name,
                    args,
                    body,
                    decorator_list,
                    returns,
                    type_comment,
                } => match name.as_str() {
                    "process" => {
                        for _param in args.args.clone() {
                            let param = _param.node;
                            match param.annotation {
                                None => panic!(),
                                Some(_annotation) => {
                                    let annotation = _annotation.node;
                                    let name_path = &get_name_path(&annotation);
                                    let schema = content
                                        .schemas
                                        .find(name_path, Some(plugin.as_ref()))
                                        .unwrap();
                                    input.insert(param.arg, Arc::downgrade(schema));
                                }
                            }
                        }
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
        resource::container::File {
            val: DataOperator::new(operator::Input::new(input)),
            std: resource::container::Std::new(class.name, Some(plugin), true),
            dir: resource::container::Dir::new(local_path, true),
        }
    }
}
