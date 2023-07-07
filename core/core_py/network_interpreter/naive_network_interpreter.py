from typing import *


class NaiveNetworkInterpretor:
    def __init__(self):
        self.executions=list[Callable]()
    def up_main_dataflow(self, dataflow:Dataflow):
        for branch in reversed(dataflow.branches):
            self.up_effect_node(branch)
        start=dataflow.start
        if isinstance(start,InputData):
            return
        elif isinstance(start, InitData):
            self.up_init_data(start)
        elif isinstance(start,Reform):
            for dataflow in start.inputs:
                self.up_main_dataflow(dataflow)
        else:raise Exception()
    def up_effect_node(self,node:EffectNode):
        if isinstance(node,InterfaceNode):
            self.executions.append(node.refer_data_ports)
        elif isinstance(node,OperatorNode):
            self.executions.append(node.execute)
        else:raise Exception()
    def up_init_data(self, node:InitData):
        self.executions.append(node.init_data)
    def execute(self,network:Network):
        self.executions.clear()
        self.up_main_dataflow(network.output)
        for execution in reversed(self.executions):
            execution()
