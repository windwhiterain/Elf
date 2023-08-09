import structs
from common import Ref
class Context:
    def __init__(self):
        self.__elf_data_0:taichi.Field=Ref(None)
        self.__elf_data_1:taichi.Field=Ref(None)
        self.__elf_interface_0:structs.__elf_ref_struct_1=structs.__elf_ref_struct_1()
