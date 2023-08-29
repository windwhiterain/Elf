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
from ui.widgets.schema_tree import SchemaTree
from ui.widgets.window import Window
from ui.palette import tool_bar,main_window
from ui.help import *
env_path = settings["python_interpreter_path"]
plugin_path = file_path+"/../../plugin"


context = elf_rust.Context(env_path, plugin_path)
context.load_resource()

class Debuger:
    def __init__(self) -> None:
        self.debug_windows=list[QWidget]()
    def debug_schema(self,context,id:int):
        infor=context.schema_infor(id)
        schema_tree = SchemaTree()
        schema_tree.set_schema(infor)
        schema_tree.show()
        self.debug_windows.append(schema_tree)
debuger=Debuger()
class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Elf")
        self.resize(1000,600)
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
            QToolButton:hover{
                border-color:"""+tool_bar.focus.get().name()+""";
            }
        """)
        self.addToolBar(toolbar)
        self.setStyleSheet("""
            QMainWindow{
                background-color:"""+main_window.bg.get().name()+""";
            }
        """)
        resource_tree = ResourceTree()
        resource_tree.refresh(context.resource_infor(),context,debuger)
        window=Window()
        window.add_tab(resource_tree,"resource")
        self.setCentralWidget(window)


app = QApplication(sys.argv)
Window = MainWindow()
Window.show()
app.exec()
