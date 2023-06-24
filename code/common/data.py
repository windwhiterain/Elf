import taichi
from typing import Any
class Data:
    def __init__(self,t:type,shape:list[int],value:Any):
        self.t=t
        self.shape=shape
        self.value=value