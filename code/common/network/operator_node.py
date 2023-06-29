from code.common.network.effect_node import EffectNode
from code.common.operator_resource import Operator
class OperatorNode(EffectNode):
    def __init__(self,operator:Operator):
        super().__init__()
        self.operator=operator
    def execute(self):
        self.operator.executable.execute(**self.inputs)