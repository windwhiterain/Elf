from PySide6 import QtCore, QtGui, QtWidgets
def set_geo(widget:QtWidgets,center:tuple[int,int],size:tuple[int,int]):
    widget.setGeometry(center[0]-size[0]/2,center[1]-size[1]/2,size[0],size[1])