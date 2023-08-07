from typing import *


class Dependency:
    def __init__(self) -> 'Dependency':
        self.nodes = list[int]()
        self.action: Optional[Callable[[Any], 'Dependency']] = None


class Node:
    def __init__(self) -> 'Node':
        self.curent_dependency: Optional[Dependency]
        self.completed = False


class Graph:
    def __init__(self) -> 'Graph':
        self.context: Any = None
        self.nodes = list[Node]()

    def solve(self, outs: list[int]):
        for out in outs:
            while True:
                node = self.nodes[out]
                if node.completed:
                    break
                dependency = node.curent_dependency
                self.solve(dependency.nodes)
                if dependency.action is not None:
                    dependency.action(self.context)
