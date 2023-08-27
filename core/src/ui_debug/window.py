from ui.palette import default
from ui.widgets.schema_tree import SchemaTree
from ui.widgets import schema_tree
import sys
from PySide6 import QtWidgets

schema_tree_color = schema_tree.ColorSheet(default.white, default.black, [
                                           default.red, default.blue, default.yellow])


class MainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()

        self.schema_tree = SchemaTree(schema_tree_color)
        self.schema_tree.set_schema(infor)
        self.setCentralWidget(self.schema_tree)


app = QtWidgets.QApplication(sys.argv)
window = MainWindow()
window.show()
app.exec()
