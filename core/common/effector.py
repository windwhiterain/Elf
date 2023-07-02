from core.common.schema import Schema
class Effector:
    def __init__(self,inputs_schemas:dict[str,Schema]):
        self.input_schemas=inputs_schemas
