from ui.widgets.color_block import *
from PySide6 import QtWidgets, QtGui
from PySide6.QtWidgets import QTabWidget
from PySide6.QtWidgets import QSizePolicy
from ui.palette import window
from ui.help import *
class Window(QWidget):
    def __init__(self):
        super().__init__()
        self.color_sheet=window
        self.tabs=QTabWidget()
        patette=self.tabs.palette()
        patette.setColor(QPalette.ColorRole.Window,self.color_sheet.bg0.get())
        self.tabs.setPalette(patette)
        self.tabs.setAutoFillBackground(True)
        self.tabs.setStyleSheet("""
            QTabBar::tab {
                background-color: """+self.color_sheet.bg1.get().name()+""";
                color:"""+self.color_sheet.text.get().name()+""";
                border-style:solid;
                border-width:2;
                border-color:"""+self.color_sheet.frame.get().name()+""";
                padding:2;
            }
            QTabWidget::pane {
            }
        """)
        self.tabs.setGeometry(self.geometry())
        self.tabs.setParent(self)
    def add_tab(self,widget:QWidget,name:str):
        self.tabs.addTab(widget,name)