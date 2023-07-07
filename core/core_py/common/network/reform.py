from core.core_py.common.interface import Interface
from core.core_py.common.data import Data
from core.core_py.common.network.dataflow_node import DataflowNode


class Reform(DataflowNode):
    """
    Attributes:
        datas:datas duplicated by this node
    """
    def __init__(self):
        super().__init__()
        self.interfaces=list[Interface]
        self.datas=list[Data]()
