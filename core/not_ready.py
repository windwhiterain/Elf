import taichi as ti
from functools import reduce
from operator import mul,add
from abc import ABC, abstractmethod
import ast
from typing import *
class Data:
    def __init__(self,field_name_chain:str,data_type_name:str,shape:list[int]):
        self.field_name_chain=field_name_chain
        self.type_name=data_type_name
        self.shape=shape
    def father(self, field_name:str, shape:list[int])-> 'Data':
        return Data(field_name + "_" + self.field_name_chain, self.type_name, shape + self.shape)
    def final_id(self, var_name:str, shape_index:list[int])->str:
        ret=var_name+"_"+self.field_name_chain
        for i in shape_index:
            ret+="_"+str(i)
        return ret
    def all_final_id(self, var_name:str)->list[str]:
        ret=[]
        shape_index= [0] * len(self.shape)
        while True:
            ret.append(self.final_id(var_name, shape_index))
            inc=False
            for i in range(len(shape_index)):
                if shape_index[i]==self.shape[i]-1:
                    shape_index[i]=0
                else:
                    shape_index[i]+=1
                    inc=True
                    break
            if not inc:
                break
        return ret
    def access_code(self,var_name:str,shape_index:list[int],index:ast.expr)->ast.Subscript:
        id=self.final_id(var_name,shape_index)
        return ast.Subscript(
            value=ast.Name(id=id),
            slice=index
        )
class Field:
    def __init__(self,id:str,interface:'Interface',shape:list[int]):
        self.id=id
        self.interface=interface
        self.shape=shape
    def all_data(self)->list[Data]:
        return [data.father(self.id,self.shape) for data in self.interface.all_data()]
class Interface(ABC):
    def __init__(self):
        self.data_dict=dict[str,Data]()
    def all_final_id(self, var_name:str)->list[str]:
        if len(self.data_dict)==0:
            self.all_data()
        return reduce(add, [data.all_final_id(var_name) for data in self.all_data()])
    @abstractmethod
    def all_data(self)->list[Data]:pass
class CompoundInterface(Interface):
    def __init__(self,field_list:list[Field]):
        super().__init__()
        self.field_dict=dict[str,Field]([(field.id,field) for field in field_list])
    def all_data(self)->list[Data]:
        if len(self.data_dict)==0:
            for data in reduce(add,[field.all_data() for key,field in self.field_dict.items()]):
                self.data_dict[data.field_name_chain]=data
        return [data for key,data in self.data_dict.items()]
class PrimitiveInterface(Interface):
    def __init__(self,data_type_name:str):
        super().__init__()
        self.data=Data("",data_type_name,[])
        self.data_dict[self.data.field_name_chain]=self.data
    def all_data(self) ->list[Data]:
        return [self.data]


