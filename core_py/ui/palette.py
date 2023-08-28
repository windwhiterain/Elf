from PySide6.QtGui import QColor
from typing import *



class ColorLink:
    def __init__(self,link_to:Union['ColorLink',QColor]):
        self.link_to=link_to
    def get(self):
        cur=self.link_to
        while isinstance(cur,ColorLink):
            cur=cur.link_to
        return cur

class Palette:
    def __init__(self):
        self.white=ColorLink(QColor(230, 210, 190))
        self.black=ColorLink(QColor(5, 10, 3))
        self.red=ColorLink(QColor(240, 120, 15))
        self.blue=ColorLink(QColor(15, 120, 240))
        self.green=ColorLink(QColor(15, 240, 120))
        self.yellow=ColorLink(QColor(150, 150, 15))
        self.white_dark=ColorLink(QColor(120, 100, 150))
        self.black_dark=ColorLink(QColor(0, 1, 0))
        self.red_dark=ColorLink(QColor(50, 20, 5))
        self.blue_dark=ColorLink(QColor(5, 20, 50))
        self.green_dark=ColorLink(QColor(5, 50, 20))
        self.yellow_dark=ColorLink(QColor(35, 35, 5))
default=Palette()

class ResourceTree:
    def __init__(self,plugin:ColorLink,schema:ColorLink,data_operator:ColorLink,bg:ColorLink,text:ColorLink) -> None:
        self.plugin=ColorLink(plugin)
        self.schema=ColorLink(schema)
        self.data_operator=ColorLink(data_operator)
        self.bg=ColorLink(bg)
        self.text=ColorLink(text)
resource_tree=ResourceTree(default.blue_dark,default.green_dark,default.red_dark,default.black,default.white)

class Window:
    def __init__(self,bg0:ColorLink,bg1:ColorLink,text:ColorLink,frame:ColorLink):
        self.bg0=ColorLink(bg0)
        self.bg1=ColorLink(bg1)
        self.text=ColorLink(text)
        self.frame=ColorLink(frame)
window=Window(default.black_dark,default.black,default.white,default.blue)

class ToolBar:
    def __init__(self,bg0:ColorLink,bg1:ColorLink,text:ColorLink,frame:ColorLink):
        self.bg0=ColorLink(bg0)
        self.bg1=ColorLink(bg1)
        self.text=ColorLink(text)
        self.frame=ColorLink(frame)
tool_bar=ToolBar(default.black_dark,default.black,default.white,default.blue)

