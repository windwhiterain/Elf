from typing import Union,Iterable,Generic,TypeVar,Any
from code.common import Ref

class StructureIndex:
    def __init__(self,value:int):
        self.value=value
class Structure:
    __fields:Union[dict[str, 'Structure'],StructureIndex]
    is_end:bool
    def __init__(self,fields:dict[str,'Structure']=None):
        from copy import deepcopy
        if fields is not None:
            for name,structure in fields.items():
                fields[name]=deepcopy(structure)
            self.is_end=False
            self.fields=fields
        else:
            self.is_end=True
            self.fields=StructureIndex(0)
        self.count=0
        for end in self.ends():
            self.count+=1
    def ends(self)->Iterable['Structure']:
        if self.is_end:
            yield self
        else:
            for name,structure in self.__fields:
                for end in structure.ends():
                    yield end
    def index(self)->StructureIndex:
        assert self.is_end
        return self.__fields
    def field(self,id)->'Structure':
        assert not self.is_end
        return self.__fields[id]
ValueType=TypeVar("ValueType",bound=Any)
class StructureList(Generic[ValueType]):
    def __init__(self,structure:Structure):
        self.list=[None]*structure.count
    def __getitem__(self, key:StructureIndex)->ValueType:
        return self.list[key.value]
    def __setitem__(self, key:StructureIndex, value:ValueType):
        self.list[key.value]=value
