from ui.widgets.color_block import *
from PySide6 import QtWidgets, QtGui
from ui.help import *
from ui.palette import resource_tree
from ui.palette import ColorLink
from PySide6.QtWidgets import QSizePolicy

class ResourceTree(QtWidgets.QLabel):
    node_height = 20

    def __init__(self):
        super().__init__()
        self.color_sheet=resource_tree
    def add_resource(self,resource,offset:float,color:ColorLink,callback=lambda :()):
        button = QtWidgets.QPushButton()
        button.setParent(self)
        button.setText(resource.name)
        button.setStyleSheet("""
            QPushButton {
                background-color: """+color.get().name()+""";
                color: """+self.color_sheet.text.get().name()+""";
                text-align: left;
            }
            QPushButton:hover {
                border-style: solid;
                border-width: 2px;
                border-color: """+self.color_sheet.focus.get().name()+"""; 
            }
        """)
        button.clicked.connect(callback)
        set_geof(button, ((self.width()+offset)/2, (self.index+0.5)*self.node_height),
                (self.width()-offset, self.node_height))
        self.index+=1
    def refresh(self, infor,context,debuger):
        set_color(self,self.color_sheet.bg)
        self.index=0
        for plugin in infor.plugins:
            self.add_resource(plugin,0,self.color_sheet.plugin)
            for schema in plugin.schemas:
                def gen_debug(id):
                    return lambda:debuger.debug_schema(context,id)
                self.add_resource(schema,10,self.color_sheet.schema,gen_debug(schema.id))
            for data_operator in plugin.data_operators:
                self.add_resource(data_operator,10,self.color_sheet.data_operator)
