from code.common.network.node import Node
from typing import *
class OutputData(Node):
    def __init__(self):
        super().__init__()
        from code.common.network.dataflow import Dataflow
        self.dataflow:Optional[Dataflow]=None