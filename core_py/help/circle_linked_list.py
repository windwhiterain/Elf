from typing import *
T=TypeVar("T",bound=Any)
class CircleLinkedList(Generic[T]):
    def __init__(self,ref:T):
        self.ref=ref
        self.next=self
        self.identity=object()
    def Union(self, other: 'CircleLinkedList[T]'):
        if self.identity is other.identity:return
        self.next=other.next
        other.next=self
        for i in self.selfs():
            i.identity=self.identity
    def selfs(self)->Iterable['CircleLinkedList[T]']:
        yield self
        ret = self.next
        while ret is not self:
            yield ret