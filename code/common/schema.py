from code.common.structure import Structure,StructureList
from code.common.shape_constrain import ShapeConstrain
from code.common.data import Data
from typing import Optional,Any
class Scheme:
    class Infor:
        def __init__(self,shape_constrain:ShapeConstrain,data:Data):
            self.shape_constrain=shape_constrain
            self.data=data
    def __init__(self,structure:Structure[Any],infors:StructureList[Infor]):
        self.structure=structure
        self.infor=infors