from common import Node,Dependency
from context import Context
import operators
def gen_nodes():
    ret=[]
    def func(context:Context):
        operators.__elf_data_operator_0.process(complex=context.__elf_interface_0.get_end())
    ret.push(Node(Dependency([],func)))
    del func
