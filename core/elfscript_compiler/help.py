import ast
from typing import *
def better_attribute_from(expr:ast.expr)->tuple[ast.expr,list[str]]:
    end=expr
    attrs=list[str]()
    while True:
        if isinstance(end,ast.Attribute):
            end=end.value
            attrs.append(expr.attr)
        else:break
    attrs.reverse()
    return end,attrs

def id_list_from(expr:Union[ast.Attribute,ast.Name])->list[str]:
    end,ids=better_attribute_from(expr)
    assert isinstance(end,ast.Name)
    ids.insert(0,end.id)
    return ids

def if_in_namespace(ids:list[str],expr:ast.expr)->Optional[list[str]]:
    end,attrs=better_attribute_from(expr)
    if isinstance(end,ast.Name):
        if end.id in ids:
            return attrs
    return None

def better_attribute_to(expr:ast.expr,attrs:list[str])->ast.expr:
    for attr in attrs:
        expr=ast.Attribute(
            value=expr,
            attr=attr
        )
    return expr
def only_target_from(ass:Union[ast.AnnAssign,ast.Assign])->ast.expr:
    if isinstance(ass,ast.AnnAssign):return ass.target
    elif isinstance(ass,ast.Assign):
        assert len(ass.targets)==1
        return ass.targets[0]
    else:raise Exception()