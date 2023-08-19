import sys
import os
from PySide6.QtWidgets import (
    QMainWindow, QApplication,
    QLabel, QToolBar, QStatusBar
)
from PySide6.QtGui import QAction, QIcon
from PySide6.QtCore import Qt
from help import *
import command


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Elf")
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
