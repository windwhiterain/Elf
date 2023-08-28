from PySide6.QtWidgets import QWidget
from PySide6.QtGui import QPalette
from ui.palette import ColorLink
class ColorBlock(QWidget):
    def __init__(self, color:ColorLink):
        super().__init__()
        self.setAutoFillBackground(True)
        palette = self.palette()
        palette.setColor(QPalette.Window, color.get())
        self.setPalette(palette)