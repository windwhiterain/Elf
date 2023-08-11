from typing import Optional, Callable


class Dependency:
    def __init__(self, nodes: list[int], action: Optional[Callable[['Context'], 'Dependency']]):
        self.nodes = nodes
        self.action = action


class Node:
    def __init__(self, dependency: Dependency):
        self.curent_dependency = dependency
        self.completed = False
class ShapeConstraint:
    def __init__(self,shape:tuple):
        self.shape=shape
