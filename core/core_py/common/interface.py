from core.core_py.common.schema import Schema
from typing import Union
from core.core_py.common.structure import StructureList
from core.core_py.common.data import Data
from core.core_py.common.context import Context

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
    def find_end(self)->Data:
        data_port=self
        while not data_port.is_end:
            data_port=data_port.ref()
        return data_port.end()

from core.core_py.common.network.effect_node import EffectNode
class Interface(EffectNode):
    def __init__(self, schema:Schema):
        super().__init__()
        self.schema=schema
        self.data_ports=StructureList[DataPort](schema.structure)
    def generate_context(self)->Context:
        return self.__generate_context(self.schema.structure)
    def __generate_context(self, structure:Schema.Structure)->Context:
        ret=structure.reference().get()()
        for name,field in structure.name_fields():
            if field.is_end:
                value=self.data_ports[field.index()].find_end()
            else:
                value=self.__generate_context(field)
            ret.__setattr__(name,value)
        return ret







