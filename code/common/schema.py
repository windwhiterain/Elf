from code.common.structure import Structure,StructureList
from code.common.shape_constraint import ShapeConstraint
from code.common.data import Data
from code.common.context import Context
from typing import Optional,Any
class Schema:
    Structure = Structure[Data.Descriptor, type[Context]]
    class Infor:
        def __init__(self, shape_constrain:ShapeConstraint, readonly:bool):
            self.shape_constrain=shape_constrain
            self.read_only=readonly
    def __init__(self, structure: 'Schema.Structure', infors:StructureList[Infor]):
        self.structure=structure
        self.infor=infors
