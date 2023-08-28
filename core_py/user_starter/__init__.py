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

        ])
        self.addToolBar(toolbar)
        resource_tree = ResourceTree()
        resource_tree.refresh(context.resource_infor())
        self.setCentralWidget(resource_tree)


app = QApplication(sys.argv)
window = MainWindow()
window.show()
app.exec()
