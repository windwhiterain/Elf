from code.common.network.node import Node
from code.common.network.dataflow import Dataflow
from code.common.interface import Interface
from typing import Optional
class InputData(Node):
    def __init__(self):
        super().__init__()
        self.dataflow:Optional[Dataflow]=None
        self.interfaces:dict[str,Interface]