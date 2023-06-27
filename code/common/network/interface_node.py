from code.common.network.node import Node
from code.common.interface import Interface
class InterfaceNode(Node):
    def __init__(self,delete:list[Interface],append:list[Interface]):
        super().__init__()
        self.delete=delete
        self.append=append