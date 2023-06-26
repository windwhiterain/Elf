from typing import Union,Iterable,Generic,TypeVar,Any
from code.common import Ref

class StructureIndex:
    def __init__(self,value:int):
        self.value=value
RefType=TypeVar("RefType",bound=Any)



class Structure(Generic[RefType]):
    class EndInfor:
        def __init__(self, index: StructureIndex, ref: Ref[RefType]):
            self.index = index
            self.ref = ref

    __fields:Union[dict[str, 'Structure'],EndInfor]

    is_end:bool
    def construct(self,fields:dict[str,'Structure']=None):
        from copy import deepcopy
        if fields is not None:
            for name,structure in fields.items():
                fields[name]=structure.__ref_copy()
            self.is_end=False
            self.__fields=fields
        else:
            self.is_end=True
            self.__fields=self.EndInfor(StructureIndex(0),Ref(None))
        self.count=0
        for end in self.ends():
            end.__fields.index = StructureIndex(self.count)
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
        return self.__fields.index
    def ref(self)->Ref[RefType]:
        assert self.is_end
        return self.__fields.ref
    def field(self,id)->'Structure':
        assert not self.is_end
        return self.__fields[id]
    def __ref_copy(self)->'Structure'[RefType]:
        ret=Structure()
        ret.is_end = self.is_end
        if self.is_end:
            ret.__fields=self.EndInfor(self.__fields.index,self.__fields.ref)
        else:
            ret.__fields=dict[str, 'Structure']()
            for name,structure in self.__fields:
                ret.__fields[name]=structure.__ref_copy()
        return ret

ValueType=TypeVar("ValueType",bound=Any)
class StructureList(Generic[ValueType]):
    def __init__(self,structure:Structure):
        self.list=[None]*structure.count
    def __getitem__(self, key:StructureIndex)->ValueType:
        return self.list[key.value]
    def __setitem__(self, key:StructureIndex, value:ValueType):
        self.list[key.value]=value
