from taichi import *
from taichi.math import *


@schema
class IntFloat2:
    ints: int[1]
    float2s: test_plugin2.Float2
    all_sc: shape_constraint(ints, float2s.all_sc)


@data_operator
class Modify(elf.Operator):
    def process(self, complex: test_plugin2.Complex):
        for index in ndrange(complex.line_sc.shape):
            complex.ff.a[index] = 1.0
            complex.ff.b[index] = 2.0
            complex.ints[index] = 3
