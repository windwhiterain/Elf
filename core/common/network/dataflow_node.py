from core.common.network.node import Node
class DataflowNode(Node):
    def __init__(self):
        from core.common.network.dataflow import Dataflow
        super().__init__()
        self.inputs=list[Dataflow]()
        self.outputs=list[Dataflow]()