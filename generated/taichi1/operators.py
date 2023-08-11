from .help import import_module_by_path
from os import path
module=import_module_by_path(path.dirname(path.abspath(__file__))+"/test_plugin1\__init__.py")
_elf_data_operator_0=module.Modify()
