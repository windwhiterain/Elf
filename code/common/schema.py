from code.common.structure import Structure,StructureList
from code.common.shape_constrain import ShapeConstrain
from code.common.data import Data
from typing import Optional,Any
class Scheme:
    class Infor:
        def __init__(self,shape_constrain:ShapeConstrain,readonly:bool):
            self.shape_constrain=shape_constrain
            self.read_only=readonly
    def __init__(self,structure:Structure[Data],infors:StructureList[Infor]):
        self.structure=structure
        self.infor=infors