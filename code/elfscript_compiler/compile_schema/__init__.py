from ast import *
from ..common import *
from code.common import *
from typing import *
import meta
def compile_schema(snippet:Decorated.Snippet)->type[Schema]:
    elf_id,classdef=snippet.elf_ids,snippet.classdef
    new_body=list[stmt]()
    new_args=list[arg]()
    class Field:
        def __init__(self,dim:int,node:expr,sc:Optional[str]):
            self.dim=dim
            self.node=node
            self.sc=sc
    fields=dict[str,Field]()
    scs=dict[str,int]()
    for _stmt in classdef.body:
        if isinstance(_stmt,AnnAssign):
            ann:AnnAssign=_stmt
            if not isinstance(ann.value,Name):raise Exception()
            name=ann.value.id
            if isinstance(ann.annotation,Subscript):
                subscript:Subscript=ann.annotation
                if not isinstance(subscript.slice,Constant):raise Exception()
                fields[name]=Field(subscript.slice.value,subscript.value,None)
            else:
                fields[name]=Field(0,ann.annotation,None)
        elif isinstance(_stmt,Assign):
            ass:Assign=_stmt
            if not isinstance(ass.value,Name):raise Exception()
            name=ass.value.id
            if not isinstance(ass.value,Call):raise Exception()
            call=ass.value
            func=call.func
            args=call.args
            attrs=if_in_namespace(elf_id,func)
            if attrs is None:raise Exception()
            if len(attrs)!=1:raise Exception()
            if attrs[1]==meta.ShapeConstrain.__name__:
                sc_dim=-1
                for _arg in args:
                    if not isinstance(_arg,Name):raise Exception()
                    new_sc_dim=fields[_arg.id].dim
                    if sc_dim!=-1 and new_sc_dim!=sc_dim:raise Exception()
                    sc_dim=new_sc_dim
                    fields[_arg.id].sc=name
                scs[name]=sc_dim
            else:raise Exception()
            

