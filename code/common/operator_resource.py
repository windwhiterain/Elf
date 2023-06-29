from code.common.schema import Schema
from code.common.resource import Resource
from code.common.executable import Executable
from code.common.effector import Effector
class Operator(Effector):
    from code.common.resource import Resource
    def __init__(self, effector:Effector.Descriptor, executable:Executable):
        super().__init__(effector)
        self.executable=executable