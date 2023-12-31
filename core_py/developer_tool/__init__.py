import sys
import os
from PySide6.QtWidgets import (
    QMainWindow, QApplication,
    QLabel, QToolBar, QStatusBar
)
from PySide6.QtGui import QAction, QIcon
from PySide6.QtCore import Qt
file_path = os.path.dirname(__file__)
core_path = file_path+"/../../core_py"
sys.path.append(core_path)
from ui.help import *
import command


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Elf-developer_tool")
        toolbar = QToolBar()
        toolbar.addActions([
            newQAction("compile", self, command.compile),
            newQAction("run", self, command.run)
        ])
        self.addToolBar(toolbar)


app = QApplication(sys.argv)
window = MainWindow()
window.show()
app.exec()
