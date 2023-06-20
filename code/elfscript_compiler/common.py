import ast
from typing import *
class Decorated:
    class Snippet:
        def __init__(self,elf_ids:list[str],classdef:ast.ClassDef):
            self.elf_ids=elf_ids
            self.classdef = classdef
    def __init__(self):
        self.schemas=list[self.Snippet]()
        self.operators=list[self.Snippet]()
def better_attribute_ast(expr:ast.expr)->tuple[ast.expr,list[str]]:
    end=expr
    attrs=list[str]()
    while True:
        if isinstance(expr,ast.Attribute):
            end=expr.value
            attrs.append(expr.attr)
        else:break
    return end,attrs

def if_in_namespace(ids:list[str],expr:ast.expr)->Optional[list[str]]:
    end,attrs=better_attribute_ast(expr)
    if isinstance(end,ast.Name):
        if end.id in ids:
            return attrs
    return None