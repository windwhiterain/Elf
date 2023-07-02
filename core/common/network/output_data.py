from core.common.network.node import Node
from typing import *
class OutputData(Node):
    def __init__(self):
        super().__init__()
        from core.common.network.dataflow import Dataflow
        self.dataflow:Optional[Dataflow]=None