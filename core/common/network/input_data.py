from core.common.network.node import Node
from core.common.network.dataflow import Dataflow
from core.common.interface import Interface
from typing import Optional
class InputData(Node):
    def __init__(self):
        super().__init__()
        self.dataflow:Optional[Dataflow]=None
        self.interfaces:dict[str,Interface]