from code.common.network.effect_node import EffectNode
from code.common.interface import Interface
from code.common.interface_creator import InterfaceCreator
class InterfaceNode(EffectNode):
    def __init__(self,creator:InterfaceCreator):
        super().__init__()
        self.creator=creator
        self.interface=Interface(creator.output_schema)
    def refer_data_ports(self):
        for this_index in self.interface.data_ports.indexs():
            name,to_index=self.creator.data_port_refs[this_index.value]
            self.interface.data_ports[this_index]=self.inputs[name].data_ports[to_index]