from core.core_py.common.structure import Structure,StructureList
from core.core_py.common.data import Data
from typing import *


class Schema:

    class Infor:
        def __init__(self, data:Data.Descriptor,readonly:bool):
            self.readonly=readonly
            self.data=data
            self.shape_constraint_id=object()
    @classmethod
    def new(cls,fields:dict[str,tuple[Union['Schema',Infor],bool]],shape_constraints:dict[str,set[object]]):
        structures={}
        constraint_ids = {}
        for name,(field,readonly) in fields.items():
            if isinstance(field,Schema):
                constraint_ids|=field.shape_constraint_list
                if readonly:
                    for infor in field.infor_list:
                        infor.readonly=True
                structures[name]=(field.structure, [field.infor_list], [field.shape_constraint_list])
            elif isinstance(field,cls.Infor):
                field.readonly=readonly
                structure_list=StructureList(list=[field])
                structures[name]=(Structure.new(),[structure_list],[StructureList(count=0)])
            else:raise Exception()
        structure,lists,nlist=Structure.new_with_lists(structures,1,1,[StructureList(list=[None])])
        infors=lists[0]
        constraint_list=nlist[0]
        for name,constraint in shape_constraints.items():
            constraint_ids-=constraint
            new_id = object()
            for infor in infors:
                if infor.shape_constraint_id in constraint:
                    infor.shape_constraint_id=new_id
            constraint_ids[name]=new_id
        constraint_list[structure.nindex]=constraint_ids
        return Schema(structure,lists[0],constraint_list)
    def __init__(self, structure: Structure,
                 infor_list:StructureList[Infor], shape_constraint_list:StructureList[dict[str,object]]):
        super().__init__()
        self.structure=structure
        self.infor_list=infor_list
        self.shape_constraint_list=shape_constraint_list
    def infor(self,ids:Iterable[str])->Infor:
        return self.infor_list[self.structure.path_field(ids).index]
    def shape_constraint(self,ids:list[str])->object:
        return self.shape_constraint_list[self.structure.path_field(ids[0:len(ids) - 1]).nindex][ids[len(ids)]]
