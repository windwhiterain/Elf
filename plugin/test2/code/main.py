from taichi import *
from taichi.math import *


@schema
class Float2:
    a: float[2]
    b: float[2]
    all_sc: shape_constraint(a, b)


@schema
class Complex:
    ff: Float2
    ints: int[2]
    line_sc: shape_constraint(ff.all_sc, ints)
    mod: int[0]
