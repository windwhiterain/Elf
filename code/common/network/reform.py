from code.common.interface import Interface
from code.common.data import Data
from code.common.network.dataflow_node import DataflowNode
import taichi
class Reform(DataflowNode):
    """
    Attributes:
        datas:datas duplicated by this node
    """
    def __init__(self):
        super().__init__()
        self.interfaces=list[Interface]
        self.datas=list[Data]()
