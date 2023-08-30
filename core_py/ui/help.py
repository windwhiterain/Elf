from PySide6 import QtCore, QtGui
from PySide6.QtWidgets import QWidget,QLabel,QTableWidget
from ui.palette import ColorLink
from PySide6.QtGui import QColor,QPalette
from PySide6.QtGui import QAction
from typing import TypeVar
def set_geo(widget:QWidget,center:tuple[int,int],size:tuple[int,int]):
    widget.setGeometry(center[0]-size[0]/2,center[1]-size[1]/2,size[0],size[1])
def set_geof(widget:QWidget,center:tuple[float,float],size:tuple[float,float]):
    widget.setGeometry(round(center[0]-size[0]/2),round(center[1]-size[1]/2),round(size[0]),round(size[1]))
def set_color(widget:QLabel,bg:ColorLink,text:ColorLink=ColorLink(QColor(255,255,255))):
    patette=widget.palette()
    patette.setColor(QLabel.backgroundRole(widget),bg.get())
    patette.setColor(QLabel.foregroundRole(widget),text.get())
    widget.setPalette(patette)
    widget.setAutoFillBackground(True)
def newQAction(text:str,parent,callback)->QAction:
    ret=QAction(text=text,parent=parent)
    ret.triggered.connect(callback)
    return ret
def tuplize(vec)->tuple[float,float]:
    return (vec.x(),vec.y())
def add(t0:tuple[float,float],t1:tuple[float,float])->tuple[float,float]:
    return (t0[0]+t1[0],t0[1]+t1[1])
def sub(t0:tuple[float,float],t1:tuple[float,float])->tuple[float,float]:
    return (t0[0]-t1[0],t0[1]-t1[1])
def center(widget:QWidget)->tuple[float,float]:
    geo=widget.geometry()
    return (geo.x()+geo.width()/2,geo.y()+geo.height()/2)
def size(widget:QWidget)->tuple[float,float]:
    return (widget.width(),widget.height())
def set_center(widget:QWidget,center:tuple[float,float]):
    set_geof(widget,center,size(widget))
def set_size(widget:QWidget,size:tuple[float,float]):
    set_geof(widget,center(widget),size)
T = TypeVar("T")
def resize(arr:list[T],size:int,gen,des)->list[T]:
    arr_len=len(arr)
    if size>arr_len:
        arr.append(gen())
        resize(arr,size,gen,des)
    elif size<len(arr):
        des(arr.pop(arr_len-1))
        resize(arr,size,gen,des)

