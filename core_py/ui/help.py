from PySide6 import QtCore, QtGui
from PySide6.QtWidgets import QWidget,QLabel
from ui.palette import ColorLink
from PySide6.QtGui import QColor
def set_geo(widget:QWidget,center:tuple[int,int],size:tuple[int,int]):
    widget.setGeometry(center[0]-size[0]/2,center[1]-size[1]/2,size[0],size[1])
def set_geof(widget:QWidget,center:tuple[float,float],size:tuple[float,float]):
    widget.setGeometry(round(center[0]-size[0]/2),round(center[1]-size[1]/2),round(size[0]),round(size[1]))
def set_color(widget:QLabel,bg:ColorLink,fg:ColorLink=ColorLink(QColor(255,255,255))):
    patette=widget.palette()
    patette.setColor(QLabel.backgroundRole(widget),bg.get())
    patette.setColor(QLabel.foregroundRole(widget),fg.get())
    widget.setPalette(patette)
    widget.setAutoFillBackground(True)