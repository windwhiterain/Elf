from code.common.structure import Structure
from code.common.plugin import Plugin
from code.common.operator import Operator_T0,Operator_T1

from typing import Generic,Any,TypeVar
T=TypeVar("T",bound=Any)
class Ref(Generic[T]):
    def __init__(self,target:T):
        self.target=target