from core.core_py.common.plugin import Plugin
from typing import *
class Resource:
    def __init__(self, plugin: Plugin, name: str,public:bool,obj:Any):
        self.plugin = Plugin
        self.name = name
        self.public=public
        self.obj=obj
