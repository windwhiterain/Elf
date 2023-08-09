import taichi
from common import Ref,ChainRef
class __elf_struct_0:
    def __init__(self):
        self.b : taichi.Field=None
        self.a : taichi.Field=None
class __elf_ref_struct_0:
    def __init__(self):
        self.b : ChainRef=None
        self.a : ChainRef=None
    def get_end(self):
        ret=__elf_struct_0()
        ret.b=self.b.get_end().value
        ret.a=self.a.get_end().value
        return ret
class __elf_struct_1:
    def __init__(self):
        self.mod : int=None
        self.ints : taichi.Field=None
        self.ff : __elf_struct_0=None
class __elf_ref_struct_1:
    def __init__(self):
        self.mod : ChainRef=None
        self.ints : ChainRef=None
        self.ff : ChainRef=None
    def get_end(self):
        ret=__elf_struct_1()
        ret.mod=self.mod.get_end().value
        ret.ints=self.ints.get_end().value
        ret.ff=self.ff.get_end()
        return ret
class __elf_struct_2:
    def __init__(self):
        self.ints : taichi.Field=None
        self.float2s : __elf_struct_0=None
class __elf_ref_struct_2:
    def __init__(self):
        self.ints : ChainRef=None
        self.float2s : ChainRef=None
    def get_end(self):
        ret=__elf_struct_2()
        ret.ints=self.ints.get_end().value
        ret.float2s=self.float2s.get_end()
        return ret
