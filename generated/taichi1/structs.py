import taichi
from .help import Ref,ChainRef
from .common import ShapeConstraint
class _elf_struct_0:
    def __init__(self):
        self.a : taichi.Field=None
        self.b : taichi.Field=None
        self.all_sc : ShapeConstraint=None
class _elf_ref_struct_0:
    def __init__(self):
        self.a : ChainRef=None
        self.b : ChainRef=None
        self.all_sc : ChainRef=None
    def get_end(self):
        ret=_elf_struct_0()
        ret.a=self.a.get_end().value
        ret.b=self.b.get_end().value
        ret.all_sc=self.all_sc.get_end().value if self.all_sc is not None else None
        return ret
class _elf_struct_1:
    def __init__(self):
        self.ints : taichi.Field=None
        self.mod : Ref=None
        self.ff : _elf_struct_0=None
        self.line_sc : ShapeConstraint=None
class _elf_ref_struct_1:
    def __init__(self):
        self.ints : ChainRef=None
        self.mod : ChainRef=None
        self.ff=_elf_ref_struct_0()
        self.line_sc : ChainRef=None
    def get_end(self):
        ret=_elf_struct_1()
        ret.ints=self.ints.get_end().value
        ret.mod=self.mod.get_end()
        ret.ff=self.ff.get_end()
        ret.line_sc=self.line_sc.get_end().value if self.line_sc is not None else None
        return ret
class _elf_struct_2:
    def __init__(self):
        self.ints : taichi.Field=None
        self.float2s : _elf_struct_0=None
        self.all_sc : ShapeConstraint=None
class _elf_ref_struct_2:
    def __init__(self):
        self.ints : ChainRef=None
        self.float2s=_elf_ref_struct_0()
        self.all_sc : ChainRef=None
    def get_end(self):
        ret=_elf_struct_2()
        ret.ints=self.ints.get_end().value
        ret.float2s=self.float2s.get_end()
        ret.all_sc=self.all_sc.get_end().value if self.all_sc is not None else None
        return ret
