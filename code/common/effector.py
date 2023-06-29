from code.common.schema import Schema
from code.common.resource import Resource
class Effector(Resource):
    class Descriptor:
        def __init__(self,resource:Resource.Descriptor,input_schemas:dict[str,Schema]):
            self.resource=resource
            self.input_schemas = input_schemas
    def __init__(self,effector:Descriptor):
        super().__init__(effector.resource)
        self.input_schemas=effector.input_schemas
