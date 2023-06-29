from code.common.resource import Resource
from code.common.effector import Effector
from code.common.schema import Schema
from code.common.interface import Interface
from typing import Callable
from code.common.structure import StructureIndex

class InterfaceCreator(Effector):
    def __init__(self,effector:Effector.Descriptor,output_schema:Schema,data_port_refs:list[tuple[str,StructureIndex]]):
        super().__init__(effector)
        self.output_schema=output_schema
        self.data_port_refs=data_port_refs