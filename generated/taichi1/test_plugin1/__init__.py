
from taichi import *
from taichi.math import *

class IntFloat2():
    pass

class Modify():

    def process(self, complex):
        complex.mod.value = 4
        assign(complex)

@kernel
def assign(complex: template()):
    for index in grouped(ndrange(*complex.line_sc.shape)):
        complex.ff.a[index] = 1.0
        complex.ff.b[index] = 2.0
        complex.ints[index] = 3
