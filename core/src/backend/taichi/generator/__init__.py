from .nodes import gen_nodes
from .context import Context


class Graph:
    def __init__(self):
        self.context = Context()
        self.nodes = gen_nodes()

    def solve(self, nodes: list[int]):
        for out in nodes:
            node = self.nodes[out]
            while not node.completed:
                dependency = node.curent_dependency
                if dependency is None:
                    break
                self.solve(dependency.nodes)
                node.curent_dependency = dependency.action(self.context)
