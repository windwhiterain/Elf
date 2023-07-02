from core.common.network.node import Node
class EffectNode(Node):
    def __init__(self):
        from core.common.interface import Interface
        super().__init__()
        self.inputs=dict[str,Interface]()