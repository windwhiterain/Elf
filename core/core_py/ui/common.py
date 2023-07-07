from PySide6.QtGui import QColor
from typing import *


class Color:
    def __init__(self,value:QColor):
        self.value=value
class ColorLink:
    def __init__(self,link_to:Union['ColorLink',Color]):
        self.link_to=link_to
    def get(self):
        cur=self
        while isinstance(cur,ColorLink):
            cur=cur.link_to
        return cur.value