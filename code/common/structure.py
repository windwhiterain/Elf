from typing import Union,Iterable,Generic,TypeVar,Any
from code.common import Ref

class StructureIndex:
    """
    Index that could only be stored in Structure and used in StructureList.
    """
    def __init__(self,value:int):
        self.value=value
RefType=TypeVar("RefType",bound=Any)



class Structure(Generic[RefType]):
    """
    An nestable accesser to any data.

    The data varied in any structure is stored in end Structure's index pointing to a StructureList.

    The data keeps the same in every reference of the same Structure is stored in end Structure's ref.

    RefType:the data type of ref.
    """
    class EndInfor:
        def __init__(self, index: StructureIndex, ref: Ref[RefType]):
            self.index = index
            self.ref = ref

    __fields:Union[dict[str, 'Structure'],EndInfor]

    is_end:bool
    def construct(self,fields:dict[str,'Structure']=None):
        """
        Actually finish construct a structure.
        :param fields: structures as fields,None if this is the end structure.
        :return:
        """
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
        """
        Find all end structures.
        :return:
        """
        if self.is_end:
            yield self
        else:
            for name,structure in self.__fields:
                for end in structure.ends():
                    yield end
    def index(self)->StructureIndex:
        """
        Get the index point to any matched StructureList.
        :return:
        """
        assert self.is_end
        return self.__fields.index
    def ref(self)->Ref[RefType]:
        """
        Get the Ref refers to the data.
        :return:
        """
        assert self.is_end
        return self.__fields.ref
    def field(self,field_name:str)->'Structure':
        """
        Get a field of this Structure by field name.
        :param field_name: field name.
        :return:
        """
        assert not self.is_end
        return self.__fields[field_name]
    def fields(self)->Iterable['Structure']:
        """
        Get all fields of this Structure.
        :return:
        """
        assert not self.is_end
        for name,structure in self.__fields:
            yield structure
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
    """
    A List accessed by Structure's Index.
    """
    def __init__(self,structure:Structure):
        """
        :param structure: the Structure access this list.
        """
        self.list=[None]*structure.count
    def __getitem__(self, key:StructureIndex)->ValueType:
        return self.list[key.value]
    def __setitem__(self, key:StructureIndex, value:ValueType):
        self.list[key.value]=value
