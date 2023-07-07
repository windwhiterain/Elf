from core.core_py.common.network.effect_node import EffectNode
from core.core_py.common.operator_resource import Operator
class OperatorNode(EffectNode):
    def __init__(self,operator:Operator):
        super().__init__()
        self.operator=operator
    def execute(self):
        self.operator.executable.execute(**self.inputs)