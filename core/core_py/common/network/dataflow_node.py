from core.core_py.common.network.node import Node
class DataflowNode(Node):
    def __init__(self):
        from core.core_py.common.network.dataflow import Dataflow
        super().__init__()
        self.inputs=list[Dataflow]()
        self.outputs=list[Dataflow]()