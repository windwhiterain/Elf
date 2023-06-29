from code.common.plugin import Plugin
class Resource:
    class Descriptor:
        def __init__(self, plugin: Plugin, name: str,public:bool):
            self.plugin = Plugin
            self.name = name
            self.public=public
    def __init__(self,resource:Descriptor):
        self.resource=resource
