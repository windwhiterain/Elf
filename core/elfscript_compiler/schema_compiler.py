from ast import *
from core.elfscript_compiler.help import *
from core.common import *
from typing import *
from core.help import *
from core.common import *
import inspect

shape_constrain_id="ShapeConstrain"
taichi_id="taichi"
def compile_schema(sytex:Any,locals:Mapping[str,Any],shape_constraint_type:type,readonly_type:type)->Schema:
    code = inspect.getsource(sytex)
    class_def=parse(code).body[0]
    assert isinstance(class_def,ClassDef)
    fields=dict[str,Union[Schema,Schema.Infor]]()
    readonlys=dict[str,bool]()
    shape_constraints=list[set[object]]()
    for name,raw_type in sytex.__annotations__.items():
        if isinstance(raw_type,tuple):
            assert isinstance(raw_type[1],int)
            _type=raw_type[0]
            dim=raw_type[1]
        else:
            _type=raw_type
            dim=0
        if hasattr(_type, "elf_schema"):
            schema = _type.elf_schema
            assert schema is not None
            fields[name] = schema
        else:
            fields[name] = Schema.Infor(Data.Descriptor(_type, dim), False)
        readonlys[name]=False
    for _stmt in class_def.body:
        if isinstance(_stmt,AnnAssign):
            pass
        elif isinstance(_stmt,Assign):
            ass:Assign=_stmt
            target=only_target_from(ass)
            assert isinstance(target,Name)
            name=target.id
            if not isinstance(ass.value,Call):raise Exception()
            call=ass.value
            func=call.func
            args=call.args
            assert eval(unparse(func),None,locals) is shape_constraint_type
            sc_dim=-1
            sc=set()
            for str_arg in args:
                assert isinstance(str_arg,Constant)
                _arg=parse(str_arg.value,mode="eval").body
                ids=id_list_from(_arg)
                assert len(ids)>0
                field = fields[ids[0]]
                if len(ids)==1:
                    assert isinstance(field,Schema.Infor)
                    sc_id=field.shape_constraint_id
                else:
                    assert isinstance(field,Schema)
                    try:
                        infor=field.infor(ids[1:])
                        sc_id=infor.shape_constraint_id
                    except:
                        sc_id=field.shape_constraint(ids[1:])
                sc.add(sc_id)
                dim=sc_id.dim
                if sc_dim==-1:sc_dim=dim
                else:assert sc_dim==dim
            shape_constraints.append(sc)
        elif isinstance(_stmt,Call):
            call=_stmt
            func = call.func
            args = call.args
            assert eval(unparse(func),None,locals) is readonly_type
            for str_arg in args:
                assert isinstance(str_arg,Constant)
                readonlys[str_arg.value]=True
        else:raise Exception()
    return Schema.new(dict([(name,(field,readonlys[name])) for name,field in fields.items()]),shape_constraints)
