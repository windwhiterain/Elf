from ui.widgets.color_block import *
from PySide6 import QtWidgets, QtGui
from ui.help import *
from ui.palette import resource_tree
from ui.palette import ColorLink
import math

class ResourceTree(QtWidgets.QLabel):
    node_height = 14

    def __init__(self):
        super().__init__()
        self.color_sheet=resource_tree
    def add_resource(self,resource,offset:float,color:ColorLink):
        lable = QtWidgets.QLabel()
        lable.setParent(self)
        set_color(lable,color)
        lable.setText(resource.name)
        set_geof(lable, ((self.width()+offset)/2, (self.index+0.5)*self.node_height),
                (self.width()-offset, self.node_height))
        self.index+=1
    def refresh(self, infor):
        set_color(self,self.color_sheet.bg)
        self.index=0
        for plugin in infor.plugins:
            self.add_resource(plugin,0,self.color_sheet.plugin)
            for schema in plugin.schemas:
                self.add_resource(schema,10,self.color_sheet.schema)
            for data_operator in plugin.data_operators:
                self.add_resource(data_operator,10,self.color_sheet.data_operator)
