from .common import Node,Dependency,ShapeConstraint
from .help import ChainRef
from .context import Context
import taichi
from . import operators
def gen_nodes()->list[Node]:
    ret=[]
    def func(context:Context):
        context._elf_data_4.value=ShapeConstraint(shape=(2,2))
    ret.append(Node(Dependency([],func)))
    del func
    def func(context:Context):
        context._elf_data_0.value=taichi.field(dtype=float,shape=context._elf_data_4.value.shape)
    ret.append(Node(Dependency([0],func)))
    del func
    def func(context:Context):
        context._elf_data_1.value=taichi.field(dtype=float,shape=context._elf_data_4.value.shape)
    ret.append(Node(Dependency([0],func)))
    del func
    def func(context:Context):
        context._elf_data_2.value=taichi.field(dtype=int,shape=context._elf_data_4.value.shape)
    ret.append(Node(Dependency([0],func)))
    del func
    def func(context:Context):
        context._elf_interface_0.ff.a=ChainRef(is_end=True,value=context._elf_data_0)
        context._elf_interface_0.ff.b=ChainRef(is_end=True,value=context._elf_data_1)
        context._elf_interface_0.ints=ChainRef(is_end=True,value=context._elf_data_2)
        context._elf_interface_0.mod=ChainRef(is_end=True,value=context._elf_data_3)
        context._elf_interface_0.line_sc=ChainRef(is_end=True,value=context._elf_data_4)
    ret.append(Node(Dependency([1,2,3],func)))
    del func
    def func(context:Context):
        operators._elf_data_operator_0.process(complex=context._elf_interface_0.get_end())
    ret.append(Node(Dependency([4],func)))
    del func
    return ret
