from code.common.interface import Interface
from code.common.network.node import Node
class Dataflow_T0:
    def __init__(self,interfaces:list[Interface]):
        self.interfaces=interfaces
class Dataflow_T1:
    def __init__(self,is_main:bool,t0:Dataflow_T0,start:Node,end:Node):
        self.is_main=is_main
        self.t0=t0
        self.start=start
        self.end=end