import taichi

from core.common.network.node import Node
from core.common.network.dataflow import Dataflow
from typing import Optional
from core.common.interface import Interface,DataPort
from core.common.schema import Schema
class InitData(Node):
    def __init__(self):
        super().__init__()
        self.dataflow:Optional[Dataflow]=None
        self.interface:Optional[Interface]=None
    def choose_schema(self,schema:Schema):
        self.interface=Interface(schema)
    def init_data(self):
        for end in self.interface.schema.structure.ends():
            data=end.end_reference().get()
            assert data.dim is not None
            if data.dim==0:
                value=data.dtype()
            elif data.dim>0:
                value=taichi.field(data.dtype, shape=data.shape_constraint_id.get())
            else:raise Exception()
            self.interface.data_ports[end.index()]=DataPort(end=value)