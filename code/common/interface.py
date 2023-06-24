from code.common.structure import Structure
from code.common.schema import Scheme
from typing import Union
from code.common.structure import StructureList
from code.common.network.node import Node
from code.common.data import Data

class DataPort:
    __value:Union['DataPort',Data]
    is_end:bool
    def __init__(self, ref: 'DataPort'=None,end:Data=None):
        if ref is not None:
            self.__value = ref
            self.is_end=False
        elif end is not None:
            self.__value=end
            self.is_end=True
        else:raise Exception("DataPort not initialized")
    def ref(self)->'DataPort':
        assert not self.is_end
        return self.__value
    def end(self)->Data:
        assert self.is_end
        return self.__value
    def get_end(self)->Data:
        data_port=self
        while not data_port.is_end:
            data_port=data_port.ref()
        return data_port.end()
class Interface(Node):
    def __init__(self,schema:Scheme):
        super().__init__()
        self.structure=schema.structure
        self.data_ports=StructureList(schema.structure)