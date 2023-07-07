class Dataflow:
    from core.core_py.common.network.dataflow_node import DataflowNode
    def __init__(self, start:DataflowNode, end:DataflowNode):
        from core.core_py.common.network.effect_node import EffectNode
        self.branches=list[EffectNode]()
        self.start=start
        self.end=end