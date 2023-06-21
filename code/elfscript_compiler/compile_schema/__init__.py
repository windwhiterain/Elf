from ast import *
from code.elfscript_compiler.common import *
from code.common import *
from typing import Optional
shape_constrain_id="ShapeConstrain"
taichi_id="taichi"
def compile_schema(snippet:Decorated.Snippet)->ClassDef:
    from copy import deepcopy
    elf_ids,classdef=snippet.elf_ids,deepcopy(snippet.classdef)
    new_body=list[stmt]()
    new_args=list[arg]()
    class Field:
        def __init__(self,dim:int,node:expr,sc:Optional[str]):
            self.dim=dim
            self.dtype=node
            self.sc=sc
    fields=dict[str,Field]()
    scs=dict[str,int]()
    for _stmt in classdef.body:
        if isinstance(_stmt,AnnAssign):
            ann:AnnAssign=_stmt
            if not isinstance(ann.target,Name):raise Exception()
            name=ann.target.id
            if isinstance(ann.annotation,Subscript):
                subscript:Subscript=ann.annotation
                if not isinstance(subscript.slice,Constant):raise Exception()
                fields[name]=Field(subscript.slice.value,subscript.value,None)
            else:
                fields[name]=Field(0,ann.annotation,None)
        elif isinstance(_stmt,Assign):
            ass:Assign=_stmt
            target=only_target_from(ass)
            assert isinstance(target,Name)
            name=target.id
            if not isinstance(ass.value,Call):raise Exception()
            call=ass.value
            func=call.func
            args=call.args
            attrs=if_in_namespace(elf_ids,func)
            if attrs is None:raise Exception()
            if len(attrs)!=1:raise Exception()
            if attrs[0]==shape_constrain_id:
                sc_dim=-1
                for _arg in args:
                    if not isinstance(_arg,Name):raise Exception()
                    new_sc_dim=fields[_arg.id].dim
                    if sc_dim!=-1 and new_sc_dim!=sc_dim:raise Exception()
                    sc_dim=new_sc_dim
                    fields[_arg.id].sc=name
                scs[name]=sc_dim
            else:raise Exception()
        else:raise Exception()
    for name,field in fields.items():
        if field.dim>0:
            shape_name=(name if field.sc is None else field.sc)+"_shape"
            value=Call(
                func=better_attribute_to(
                    Name(id=elf_ids[0]),
                    [taichi_id,"field"]
                ),
                args=[
                    field.dtype
                ],
                keywords=[
                    keyword(
                        arg='shape',
                        value=Name(id=shape_name)
                    )
                ]
            )
            if field.sc is None:
                new_args.append(arg(
                    arg=shape_name,
                    annotation=Subscript(
                        value=Name(id="tuple"),
                        slice=Tuple(elts=[Name(id="int")]*field.dim)
                    )
                ))

        elif field.dim==0:
            value=Call(
                func=field.dtype,
                args=[],
                keywords=[]
            )
            new_args.append(arg(
                arg=name,
                annotation=field.dtype
            ))
        else:raise Exception()
        new_body.append(
            Assign(
                targets=[Attribute(
                    value=Name(id="self"),
                    attr=name
                )],
                value=value
            )
        )
    for name,dim in scs.items():
        new_args.append(arg(
            arg=name+"_shape",
            annotation=Subscript(
                value=Name(id="tuple"),
                slice=ast.Tuple(elts=[Name(id="int")] * dim)
            )
        ))
    classdef.body=[
        FunctionDef(
            name='__init__',
            args=new_args,
            body=new_body,
            decorator_list=[]
        )
    ]
    return classdef

if __name__=='__main__':
    with open("./test_code","r") as f:
        code=f.read()
    snippet=Decorated.Snippet(["elf"],parse(code).body[0])
    res=compile_schema(snippet)
    print(unparse(fix_missing_locations(res)))


