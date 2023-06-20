import meta as elf
from taichi import *
from taichi.math import *
@elf.schema
@data_oriented
class Ray:
    start:vec3[1]#field primitive,dimension is 1
    direction:vec3[1]
    all_sc=elf.ShapeConstrain(start,direction)
@elf.schema
@data_oriented
class Light:
    ray:Ray#compound:refers to Ray
    energe:float[1]
    line_sc=elf.ShapeConstrain(ray.all_sc,energe)
    density:float[3]#dimension is 3
    mode:int#variable primitive
@elf.operator
@kernel
class MoveLight(elf.Operator):
    def process(self,light:Light):#entry:parameter type determines the schema
        for index in ndrange(light.line_sc.shape):#get shape from ShapeConstrain
            start=light.ray.start[index]
            direction=light.ray.direction[index]
            energe=light.energe[index]
            light.ray.start[index]+=direction*energe*light.density[round(direction)]*light.mode
