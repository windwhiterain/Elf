from copy import deepcopy
from typing import *

class StructureIndex:
    """
    Index that could only be stored in Structure and used in StructureList.
    """
    def __init__(self,value:int):
        self.value=value

ValueType=TypeVar("ValueType",bound=Any)
class StructureList(Generic[ValueType]):
    """
    A List accessed by Structure's Index.
    """
    def __init__(self,count:int=None,list:list[ValueType]=None):
        if count is not None:
            self.list:list[ValueType]=[None]*count
        elif list is not None:
            self.list=list
        else:raise Exception()
    @classmethod
    def new(cls,structure:'Structure'):
        """
        :param structure: the Structure access this list.
        """
        return cls(structure.count)

    @classmethod
    def new_not_end(cls,structure:'Structure'):
        return cls(structure.ncount)
    def __getitem__(self, key:StructureIndex)->ValueType:
        return self.list[key.value]
    def __iter__(self)->Iterable[ValueType]:
        return self.list.__iter__()
    def __setitem__(self, key:StructureIndex, value:ValueType):
        self.list[key.value]=value
    def indexs(self)->Iterable[StructureIndex]:
        for i in range(len(self.list)):
            yield StructureIndex(i)
    def __iadd__(self, other):
        self.list+=other.list
        return self

class Structure:
    """
    An nestable accesser to any data.

    Attributes:

    The data varied in any structure is stored in end Structure's index pointing to a StructureList.

    The data keeps the same in every reference of the same Structure is stored in end Structure's ref.

    Generic:

    EndRefType:the data type of refs in the end structures.

    RefType:the data type of refs in all structures
    """
    class NotEnd:
        def __init__(self,fields:dict[str, 'Structure'],index:StructureIndex):
            self.fields=fields
            self.index=index
    def __init__(self,fields: Union[NotEnd, StructureIndex],is_end: bool,count:int,ncount:int):
        self.__data=fields
        self.is_end=is_end
        self.count=count
        self.ncount=ncount
    @classmethod
    def new(cls, fields:dict[str, 'Structure']=None):
        """
        Actually finish construct a structure.
        :param fields: structures as fields,None if this is the end structure.
        :return:
        """
        if fields is not None:
            for name,structure in fields.items():
                fields[name]=deepcopy(structure)
            is_end=False
            fields=fields
        else:
            is_end=True
            fields=StructureIndex(0)
        ret=cls(cls.NotEnd(fields,StructureIndex(0)),is_end,0,0)
        for end in ret.ends():
            end.index = StructureIndex(ret.count)
            ret.count+=1
        for not_end in ret.not_ends():
            not_end.nindex = StructureIndex(ret.ncount)
            ret.ncount+=1
        return ret
    def add_field(self, name:str, field: 'Structure')->tuple[StructureList[StructureIndex],StructureList[StructureIndex]]:
        assert not self.is_end
        assert name not in self.__data
        new_structure=deepcopy(field)
        self.fields_dict[name]=new_structure
        ret=StructureList[StructureIndex].new(new_structure)
        nret=StructureList[StructureIndex].new_not_end(new_structure)
        for end in new_structure.ends():
            pre=end.index
            end.index=StructureIndex(self.count)
            new=end.index
            self.count+=1
            ret[pre]=new
        for not_end in new_structure.not_ends():
            pre = not_end.nindex
            not_end.nindex = StructureIndex(self.ncount)
            new = not_end.nindex
            self.ncount += 1
            nret[pre] = new
        return ret,nret
    def merge_list(self,add_lists:list[StructureList],self_lists:list[StructureList],map_list:StructureList[StructureIndex]):
        assert len(self_lists) == len(add_lists)
        for index in range(len(self_lists)):
            self_list, add_list = self_lists[index], add_lists[index]
            self_list += add_list
            for i in map_list.indexs():
                self_list[map_list[i]] = add_list[i]
    def add_field_lists(self,name:str,field:'Structure',add_lists:list[StructureList],self_lists:list[StructureList],
                        nadd_lists:list[StructureList],nself_lists:list[StructureList]):
        map_list,nmap_list=self.add_field(name,field)
        self.merge_list(add_lists,self_lists,map_list)
        self.merge_list(nadd_lists,nself_lists,nmap_list)
    @classmethod
    def new_with_lists(cls, fields:dict[str, tuple['Structure',list[StructureList],list[StructureList]]], list_count:int, nlist_count:int,
                       self_nlist:list[StructureList]):
        ret_structure=cls(cls.NotEnd({},StructureIndex(0)),False,0,1)
        ret_lists=[StructureList(0)]*list_count
        assert len(self_nlist) == nlist_count
        nret_lists=self_nlist
        for name,(field,lists,nlists) in fields.items():
            assert len(lists)==list_count
            assert len(nlists)==nlist_count
            ret_structure.add_field_lists(name,field,lists,ret_lists,nlists,nret_lists)
        return ret_structure,ret_lists,nret_lists
    def ends(self)->Iterable['Structure']:
        """
        Find all end structures.
        :return:
        """
        if self.is_end:
            yield self
        else:
            for name,structure in self.fields():
                for end in structure.ends():
                    yield end
    def not_ends(self)->Iterable['Structure']:
        if not self.is_end:
            yield self
            for field in self.fields():
                if not field.is_end:
                    for not_end in field.not_ends():
                        yield not_end

    @property
    def index(self)->StructureIndex:
        """
        Get the index point to any matched StructureList.
        :return:
        """
        assert self.is_end
        return self.__data
    @index.setter
    def index(self,index:StructureIndex):
        assert self.is_end
        self.__data=index

    @property
    def nindex(self) -> StructureIndex:
        assert not self.is_end
        return self.__data.index

    @nindex.setter
    def nindex(self, index: StructureIndex):
        assert not self.is_end
        self.__data.index = index
    @property
    def fields_dict(self):
        assert not self.is_end
        return self.__data.fields
    def field(self,field_name:str)->'Structure':
        """
        Get a field of this Structure by field name.
        :param field_name: field name.
        :return:
        """
        return self.fields_dict[field_name]
    def path_field(self,ids:Iterable[str])->'Structure':
        current=self
        for name in ids:
            current=self.field(name)
        return current
    def fields(self)->Iterable['Structure']:
        """
        Get all fields of this Structure.
        :return:
        """
        for name,field in self.fields_dict:
            yield field
    def name_fields(self)->Iterable[tuple[str,'Structure']]:
        for name, field in self.fields_dict:
            yield name,field

