from core.common.network.output_data import OutputData
from core.common.resource import Resource
from core.common.network.init_data import InitData
class Network(Resource):
    def __init__(self,resource:Resource):
        super().__init__(resource)
        self.input=InitData()
        self.output=OutputData()
        self.inits=list[InitData]()
        self.is_operator=False
