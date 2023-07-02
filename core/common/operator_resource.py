from core.common.schema import Schema
from core.common.resource import Resource
from core.common.executable import Executable
from core.common.effector import Effector
class Operator(Effector):
    from core.common.resource import Resource
    def __init__(self,inputs_schemas:dict[str,Schema],executable:Executable):
        super().__init__(inputs_schemas)
        self.executable=executable