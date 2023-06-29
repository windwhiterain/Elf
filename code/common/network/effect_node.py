from code.common.network.node import Node
class EffectNode(Node):
    def __init__(self):
        from code.common.interface import Interface
        super().__init__()
        self.inputs=dict[str,Interface]()