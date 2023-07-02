from typing import *
from meta.constraint_decorator import ShapeConstrain,readonly
class schema:
    def __init__(self,locals:Mapping[str,Any]):
        self.locals=locals
    def __call__(self,sytex):
        import core.elfscript_compiler as compiler
        sytex.elf_schema=compiler.compile_schema(sytex,self.locals,ShapeConstrain,readonly)
        return sytex