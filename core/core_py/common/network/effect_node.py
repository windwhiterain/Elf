from core.core_py.common.network.node import Node
class EffectNode(Node):
    def __init__(self):
        from core.core_py.common.interface import Interface
        super().__init__()
        self.inputs=dict[str,Interface]()