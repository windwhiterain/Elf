from typing import *


class Dependency:
    def __init__(self) -> 'Dependency':
        self.nodes = list[int]()
        self.action: Optional[Callable[[], 'Dependency']] = None


class Node:
    def __init__(self) -> 'Node':
        self.curent_dependency: Optional[Dependency]


class Graph:
    def __init__(self) -> 'Graph':
        self.nodes = list[Node]()


def solve(graph: Graph, outs: list[int]):
    for out in outs:
        node = graph.nodes[out]
        dependency = node.curent_dependency
        if dependency is None:
            continue
        solve(graph,dependency.nodes)
        dependency.action()

