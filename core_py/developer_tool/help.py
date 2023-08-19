from PySide6.QtGui import QAction


def newQAction(text:str,parent,callback)->QAction:
    ret=QAction(text=text,parent=parent)
    ret.triggered.connect(callback)
    return ret
