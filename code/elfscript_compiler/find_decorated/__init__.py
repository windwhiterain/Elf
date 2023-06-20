from ast import *
from ..common import *
elf_id="meta"

def find_decorated(module:Module)->Decorated:
    from copy import deepcopy
    new_module=deepcopy(module)
    ret=Decorated()
    elf_ids=list[str]()
    for _stmt in new_module.body:
        if isinstance(_stmt,Import):
            _import:Import=_stmt
            for _alias in _import.names:
                if _alias.name==elf_id:
                    elf_ids.append(_alias.name)
                if _alias.asname is not None:
                    elf_ids.append(_alias.asname)
        elif isinstance(_stmt,ClassDef):
            classdef:ClassDef=_stmt
            for decorator in classdef.decorator_list:
                if isinstance(decorator,Attribute):
                    attribute=decorator.value
                    attr=decorator.attr
                    if isinstance(attribute,Name):
                        name:Name=attribute
                        if name.id in elf_ids:
                            if attr=="schema":
                                classdef.decorator_list.remove(decorator)
                                new_elf_ids = deepcopy(elf_ids)
                                ret.schemas.append(Decorated.Snippet(new_elf_ids,classdef))
                            elif attr=="operator":
                                classdef.decorator_list.remove(decorator)
                                new_elf_ids = deepcopy(elf_ids)
                                ret.operators.append(Decorated.Snippet(new_elf_ids, classdef))
    return ret

if __name__=='__main__':
    with open("../../../plugin/template/code/main.py", "r") as f:
        code = f.read()
    module = parse(code)
    res = find_decorated(module)
    for snippet in res.schemas:
        print(snippet.elf_ids)
        print(dump(snippet.classdef, indent=1))
    for snippet in res.operators:
        print(snippet.elf_ids)
        print(dump(snippet.classdef, indent=1))

