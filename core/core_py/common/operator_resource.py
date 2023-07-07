from core.core_py.common.schema import Schema
from core.core_py.common.executable import Executable
from core.core_py.common.effector import Effector
class Operator(Effector):
    def __init__(self,inputs_schemas:dict[str,Schema],executable:Executable):
        super().__init__(inputs_schemas)
        self.executable=executable