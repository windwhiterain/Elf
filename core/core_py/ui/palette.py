from ui.common import Color
from PySide6.QtGui import QColor
class Palette:
    pass
default=Palette()
default.white=Color(QColor(230, 210, 190))
default.black=Color(QColor(3, 5, 15))
default.red=Color(QColor(240, 120, 15))
default.blue=Color(QColor(15, 120, 240))
default.green=Color(QColor(15, 240, 120))
default.yellow=Color(QColor(150, 150, 15))