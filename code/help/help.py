from typing import *
T=TypeVar("T",bound=Any)
class Ref(Generic[T]):
    def __init__(self,target:T):
        self.target=target
    def get(self)->T:

        return self.target
    def set(self,value:T):
        self.target=value