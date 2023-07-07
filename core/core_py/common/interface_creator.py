from core.core_py.common.effector import Effector
from core.core_py.common.schema import Schema
from core.core_py.common.structure import StructureIndex

class InterfaceCreator(Effector):
    def __init__(self,inputs_schemas:dict[str,Schema],output_schema:Schema,data_port_refs:list[tuple[str,StructureIndex]]):
        super().__init__(inputs_schemas)
        self.output_schema=output_schema
        self.data_port_refs=data_port_refs