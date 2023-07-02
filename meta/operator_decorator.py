from core.common import *
def operator(sytex):
    assert isinstance(sytex,Executable)
    sytex.elf_operator=None
    return sytex