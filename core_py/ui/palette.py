from PySide6.QtGui import QColor
from typing import *



class ColorLink:
    def __init__(self,link_to:Union['ColorLink',QColor]):
        self.link_to=link_to
    def get(self)->QColor:
        cur=self.link_to
        while isinstance(cur,ColorLink):
            cur=cur.link_to
        return cur
    def code(self)->str:
        return self.get().name()

class Palette:
    def __init__(self):
        self.white=ColorLink(QColor(232, 234, 236))
        self.black=ColorLink(QColor(27, 29, 25))
        self.red=ColorLink(QColor(240, 120, 15))
        self.blue=ColorLink(QColor(15, 120, 240))
        self.green=ColorLink(QColor(15, 240, 120))
        self.yellow=ColorLink(QColor(150, 150, 15))
        self.purple=ColorLink(QColor(150, 15, 150))
        self.white_dark=ColorLink(QColor(100, 102, 105))
        self.black_dark=ColorLink(QColor(0, 1, 0))
        self.red_dark=ColorLink(QColor(50, 20, 5))
        self.blue_dark=ColorLink(QColor(5, 20, 50))
        self.green_dark=ColorLink(QColor(5, 50, 20))
        self.yellow_dark=ColorLink(QColor(35, 35, 5))
default=Palette()

class ResourceTree:
    def __init__(self,plugin:ColorLink,schema:ColorLink,data_operator:ColorLink,bg:ColorLink,text:ColorLink,focus:ColorLink) -> None:
        self.plugin=ColorLink(plugin)
        self.schema=ColorLink(schema)
        self.data_operator=ColorLink(data_operator)
        self.bg=ColorLink(bg)
        self.text=ColorLink(text)
        self.focus=ColorLink(focus)
resource_tree=ResourceTree(default.blue_dark,default.green_dark,default.red_dark,default.black_dark,default.white,default.white)

class Window:
    def __init__(self,bg0:ColorLink,bg1:ColorLink,text:ColorLink,frame:ColorLink,focus:ColorLink):
        self.bg0=ColorLink(bg0)
        self.bg1=ColorLink(bg1)
        self.text=ColorLink(text)
        self.frame=ColorLink(frame)
        self.focus=ColorLink(focus)
window=Window(default.black_dark,default.black,default.white,default.blue,default.white)

class ToolBar:
    def __init__(self,bg0:ColorLink,bg1:ColorLink,text:ColorLink,frame:ColorLink,focus:ColorLink):
        self.bg0=ColorLink(bg0)
        self.bg1=ColorLink(bg1)
        self.text=ColorLink(text)
        self.frame=ColorLink(frame)
        self.focus=ColorLink(focus)
tool_bar=ToolBar(default.black_dark,default.black,default.white,default.blue,default.white)

class SchemaTree:
    def __init__(self, bg0: ColorLink,bg1: ColorLink,frame:ColorLink,text:ColorLink, line: ColorLink, shape_constraints: list[ColorLink]):
        self.bg0 = ColorLink(bg0)
        self.bg1 = ColorLink(bg1)
        self.frame = ColorLink(frame)
        self.line = ColorLink(line)
        self.text = ColorLink(text)
        self.shape_constraints = [ColorLink(color)
                                  for color in shape_constraints]
schema_tree=SchemaTree(default.black_dark,default.black,default.blue,default.white,default.white,[default.red,default.yellow,default.green,default.black,default.purple])

class MainWindow:
    def __init__(self,bg:ColorLink):
        self.bg=ColorLink(bg)
main_window=MainWindow(default.white_dark)

class NetworkPanel:
    def __init__(self,bg:ColorLink):
        self.bg=ColorLink(bg)
network_panel=NetworkPanel(default.black_dark)

class ChooseList:
    def __init__(self,bg0:ColorLink,bg1:ColorLink,text:ColorLink,frame:ColorLink):
        self.bg0=ColorLink(bg0)
        self.bg1=ColorLink(bg1)
        self.text=ColorLink(text)
        self.frame=ColorLink(frame)
choose_list=ChooseList(default.black,default.black_dark,default.white,default.blue)

class Node:
    def __init__(self,bg:ColorLink,text:ColorLink,frame_data_operator:ColorLink,frame_interface_operator:ColorLink):
        self.bg=ColorLink(bg)
        self.text=ColorLink(text)
        self.frame_data_operator=ColorLink(frame_data_operator)
        self.frame_interface_operator=ColorLink(frame_interface_operator)
node=Node(default.black,default.white,default.red,default.green)