from code.common.network.dataflow import Dataflow_T1
class Node:
    def __init__(self):
        self.input_dataflow=list[Dataflow_T1]
        self.output_dataflow=list[Dataflow_T1]