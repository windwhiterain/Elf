from code.common.plugin import Plugin
from code.common.schema import Scheme
from code.common.network.node import Node
class Operator_T0:
    def __init__(self,plugin:Plugin,name:str,input_schema:Scheme,output_schema:Scheme,code_path:str):
        self.plugin=plugin
        self.name=name
        self.input_schema=input_schema
        self.input_schema=output_schema
        self.code_path=code_path
class Operator_T1(Node):
    def __init__(self,t0:Operator_T0,guid:int):
        super().__init__()
        self.guid=guid
