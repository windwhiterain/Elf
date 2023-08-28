import sys
from PySide6.QtCore import Qt
from PySide6.QtGui import QAction, QIcon
from PySide6.QtWidgets import (
    QMainWindow, QApplication,
    QLabel, QToolBar, QStatusBar
)
import elf_rust
import os
from json import load
file_path = os.path.dirname(__file__)
settings = load(open(file_path+"/../../settings.json"))
core_path = file_path+"/../../core_py"
sys.path.append(core_path)
from ui.widgets.resource_tree import ResourceTree
from ui.widgets.window import Window
from ui.palette import tool_bar
from ui.help import *
env_path = settings["python_interpreter_path"]
plugin_path = file_path+"/../../plugin"


context = elf_rust.Context(env_path, plugin_path)
context.load_resource()


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Elf")
        toolbar = QToolBar()
        toolbar.addActions([
            newQAction("reload",self,context.load_resource)
        ])
        toolbar.setStyleSheet("""
            QToolBar{
                background-color:"""+tool_bar.bg0.get().name()+""";
                padding:1;
            }
            QToolButton{
                background-color:"""+tool_bar.bg1.get().name()+""";
                padding:2;
                border-style:solid;
                border-width:2;
                border-color:"""+tool_bar.frame.get().name()+""";
                color:"""+tool_bar.text.get().name()+""";
            }
        """)
        self.addToolBar(toolbar)
        resource_tree = ResourceTree()
        resource_tree.refresh(context.resource_infor())
        window=Window()
        window.add_tab(resource_tree,"resource")
        self.setCentralWidget(window)


app = QApplication(sys.argv)
Window = MainWindow()
Window.show()
app.exec()
