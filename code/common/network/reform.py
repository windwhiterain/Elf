from code.common.network.node import Node
from code.common.interface import Interface
from code.common.data import Data
import taichi
class Reform(Node):
    def __init__(self):
        super().__init__()
        self.datas=list[Data]()
        self.interfaces=list[Interface]()#foreign key(self.output_dataflow.index)
