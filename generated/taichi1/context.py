from . import structs
from .help import Ref
class Context:
    def __init__(self):
        self._elf_data_0=Ref(None)
        self._elf_data_1=Ref(None)
        self._elf_data_2=Ref(None)
        self._elf_data_3=Ref(None)
        self._elf_data_4=Ref(None)
        self._elf_interface_0:structs._elf_ref_struct_1=structs._elf_ref_struct_1()
