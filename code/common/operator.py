from .plugin import Plugin
from .schema import Schema
class Operator:
    def __init__(self,plugin:Plugin,name:str):
        self.plugin=plugin
        self.name=name
        self.input_schema=Schema