from taichi import *
from taichi.math import *


@schema
class Ray:
    start: vec3[1]
    direction: vec3[1]
    all_sc: shape_constraint(start, direction)


@schema
class Light:
    ray: Ray
    energe: float[1]
    line_sc: shape_constraint(ray.all_sc, energe)
    density: float[3]
    mode: int[0]


@operator
class Modify(elf.Operator):
    def process(self, pair: test_plugin1.IntFloatPair):
        for index in ndrange(pair.line_sc.shape):
            pair.ints[index] += 1
            pair.floats[index] /= 2
