from code.common.data import Data
from typing import Iterable
class ShapeConstraint:
    def __init__(self,data:Data):
        self.data=data
        self.next=self
    def Union(self,constraint:'ShapeConstraint'):
        self.next=constraint.next
        constraint.next=self
    def datas(self)->Iterable[Data]:
        yield self
        ret=self.next
        while ret is not self:
            yield ret