from core.common.network.effect_node import EffectNode
from core.common.network.network import Network

class NetworkNode(EffectNode):
    def __init__(self,network:Network):
        super().__init__()
        self.network=network