class Dataflow:
    from code.common.network.dataflow_node import DataflowNode
    def __init__(self, start:DataflowNode, end:DataflowNode):
        from code.common.network.effect_node import EffectNode
        self.branches=list[EffectNode]()
        self.start=start
        self.end=end