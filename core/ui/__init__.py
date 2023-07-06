import math
import sys
from PySide6 import QtCore, QtGui, QtWidgets
from PySide6.QtCore import Qt
from typing import *
from core.ui.widgets.schema_tree import SchemaTree
from core.ui.palette import default



schema_tree_color=SchemaTree.ColorSheet(default.white, default.black,[default.red,default.blue,default.yellow])
n1=SchemaTree.Node("1",[],0)
n2=SchemaTree.Node("2",[],1)
n11=SchemaTree.Node("2",[n1,n2],2)
n12=SchemaTree.Node("2",[],2)
root=SchemaTree.Node("2",[n11,n12],2)
class MainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()

        self.schema_tree = SchemaTree(schema_tree_color)
        self.schema_tree.set_schema(root)
        self.setCentralWidget(self.schema_tree)

app = QtWidgets.QApplication(sys.argv)
window = MainWindow()
window.show()
app.exec()