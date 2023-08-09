from typing import *
import importlib.util
from context import Context
from nodes import gen_nodes


class Dependency:
    def __init__(self, nodes: list[int], action: Optional[Callable[[Context], 'Dependency']]) -> 'Dependency':
        self.nodes = nodes
        self.action = action


class Node:
    def __init__(self, dependency: Dependency) -> 'Node':
        self.curent_dependency: Optional[Dependency]
        self.completed = False


class Graph:
    def __init__(self) -> 'Graph':
        self.context = Context()
        self.nodes = gen_nodes()

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


def import_module_by_path(module_path):
    """
    根据给定的完整路径动态导入模块
    """
    spec = importlib.util.spec_from_file_location("module_name", module_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class Ref:
    def __init__(self, value):
        self.value = value


class ChainRef:
    def __init__(self, is_end: bool, value):
        self.is_end = is_end
        self.value = value

    def get_end(self):
        ret = self
        while not ret.is_end:
            ret = ret.value
        return ret.value
