from taichi import *
from taichi.math import *


@schema
class IntFloatPair:
    ints: int[1]
    floats: float[1]
    all_sc: shape_constraint(ints, floats)


@schema
class Light:
    ray: test_plugin2.Ray
    energe: float[1]
    line_sc: shape_constraint(ray.all_sc, energe)
    density: float[3]
    mode: int[0]


@elf.operator
class MoveLight(elf.Operator):
    def process(self, light: Light):
        for index in ndrange(light.line_sc.shape):
            start = light.ray.start[index]
            direction = light.ray.direction[index]
            energe = light.energe[index]
            light.ray.start[index] += direction*energe * \
                light.density[round(direction)]*light.mode
