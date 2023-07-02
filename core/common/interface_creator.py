from core.common.resource import Resource
from core.common.effector import Effector
from core.common.schema import Schema
from core.common.interface import Interface
from typing import Callable
from core.common.structure import StructureIndex

class InterfaceCreator(Effector):
    def __init__(self,inputs_schemas:dict[str,Schema],output_schema:Schema,data_port_refs:list[tuple[str,StructureIndex]]):
        super().__init__(inputs_schemas)
        self.output_schema=output_schema
        self.data_port_refs=data_port_refs