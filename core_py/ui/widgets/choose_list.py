import PySide6.QtGui
from ui.widgets.color_block import *
from PySide6 import QtWidgets, QtGui
from ui.help import *
from ui.palette import choose_list
from ui.palette import ColorLink
from PySide6.QtWidgets import QLineEdit,QVBoxLayout
from PySide6.QtCore import Qt
class ChooseList(QWidget):
    def __init__(self,callback):
        super().__init__()
        layout=QVBoxLayout()
        self.line_edit=QLineEdit()
        layout.addWidget(self.line_edit)
        self.setLayout(layout)
        self.setVisible(False)
        self.is_open=False
        self.line_edit.returnPressed.connect(lambda :(callback(),self.close()))
        self.color_sheet=choose_list
        self.setStyleSheet("""
            QWidget{
                background-color:"""+self.color_sheet.bg0.get().name()+""";
            }
            QLineEdit{
                background-color:"""+self.color_sheet.bg1.get().name()+""";
                color:"""+self.color_sheet.text.get().name()+""";
                border-style:solid;
                border-width:2px;
                border-color:"""+self.color_sheet.frame.get().name()+""";
            }
        """)
    def open(self,pos:tuple[float,float]):
        self.setGeometry(pos[0],pos[1],150,50)
        self.setVisible(True)
        self.is_open=True
        self.setFocus()
    def close(self):
        self.line_edit.setText("")
        self.setVisible(False)
        self.is_open=False
    def text(self)->str:
        return self.line_edit.text()