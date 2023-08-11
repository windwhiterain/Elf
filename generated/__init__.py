import taichi1
import taichi
taichi.init()
graph=taichi1.Graph()
graph.solve([5])
print(graph.context._elf_data_0.value)
print(graph.context._elf_data_1.value)
print(graph.context._elf_data_2.value)
print(graph.context._elf_data_3.value)

