from ui.widgets.color_block import *
from PySide6 import QtWidgets,QtGui
from ui.help import *
import elf_py
import math
RustNode=elf_py.ui.schema_tree.Node

class Node:
    def __init__(self,rust_node:RustNode):
        self.x,self.y,self.w=0.0,0.0,0.0
        self.name:str=rust_node.name
        self.childs:list[Node]=[Node(r_node) for r_node in rust_node.childs]
        self.sc_id=rust_node.sc_id
def cal_y(self:Node, y: float):
    self.y = y
    for child in self.childs:
        cal_y(child,y + SchemaTree.interval_y + SchemaTree.node_h)

def cal_w(self:Node):
    self.w-=SchemaTree.interval_x
    for child in self.childs:
        self.w +=SchemaTree.interval_x
        cal_w(child)
        self.w+=child.w
    if self.w<SchemaTree.node_w:
        self.w=SchemaTree.node_w
def cal_x(self:Node,x:float):
    self.x=x
    c_x=self.x-self.w/2
    for child in self.childs:
        c_x+=child.w/2
        cal_x(child,c_x)
        c_x+=child.w/2
        c_x+=SchemaTree.interval_x
def iter(self:Node)->Iterable[Node]:
    yield self
    for child in self.childs:
        for ret in iter(child):
            yield ret
def cal_edge(self:Node)->tuple[float,float,float,float]:
    min_x,min_y,max_x,max_y=math.inf,math.inf,-math.inf,-math.inf
    for node in iter(self):
        if node.x>max_x:max_x=node.x
        if node.x<min_x:min_x=node.x
        if node.y>max_y:max_y=node.y
        if node.y<min_y:min_y=node.y
    min_x-=SchemaTree.node_w/2
    max_x+=SchemaTree.node_w/2
    min_y-=SchemaTree.node_h/2
    max_y+=SchemaTree.node_h/2
    return (min_x,min_y,max_x,max_y)

def offset_to_z(self:Node) -> tuple[float, float]:
    min_x, min_y, max_x, max_y = cal_edge(self)
    for node in iter(self):
        node.x -= min_x
        node.y -= min_y
    return max_x - min_x, max_y - min_y
class ColorSheet:
    def __init__(self, bg: Color, line: Color, shape_constraints: list[Color]):
        self.bg = ColorLink(bg)
        self.line = ColorLink(line)
        self.shape_constraints = [ColorLink(color) for color in shape_constraints]
class SchemaTree(QtWidgets.QLabel):
    interval_x=1.0
    interval_y=0.3
    node_w=2.0
    node_h=0.5
    def __init__(self,color_sheet:ColorSheet):
        super().__init__()
        self.root:Optional[Node]=None
        self.w,self.h=0.0,0.0
        self.color_sheet = color_sheet

    def scale_to(self, w: float, h: float):
        self.sc_w, self.sc_h = w / self.w, h / self.h
    def scale(self,sc:float):
        self.sc_w=self.sc_h=sc
    def apply(self,x:float,y:float):
        return round(x*self.sc_w),round(y*self.sc_h)
    def apply_x(self,x:float):
        return round(x*self.sc_w)
    def apply_y(self,x:float):
        return round(x*self.sc_h)
    def paint_schema(self):
        w,h=self.apply(self.w,self.h)
        self.setFixedSize(w, h)
        canvas = QtGui.QPixmap(w,h)
        canvas.fill(self.color_sheet.bg.get())
        painter = QtGui.QPainter(canvas)
        for node in iter(self.root):
            label=QtWidgets.QLabel()
            label.setParent(self)
            label.setText(node.name)
            label.setAutoFillBackground(True)
            x1,y1=self.apply(node.x,node.y)
            y_mid=self.apply_y(node.y+(SchemaTree.node_h+SchemaTree.interval_y)/2)
            w,h=self.apply(SchemaTree.node_w,SchemaTree.node_h)
            set_geo(label,(x1,y1),(w,h))
            y_end=self.apply_y(self.h-SchemaTree.node_h/2)
            if len(node.childs)!=0:
                painter.drawLine(x1, y1, x1, y_mid)
                for child in node.childs:
                    x2,y2=self.apply(child.x,child.y)
                    painter.drawLine(x2,y2,x2,y_mid)
                xl, xr = self.apply_x(node.childs[0].x),self.apply_x(node.childs[-1].x)
                painter.drawLine(xl,y_mid,xr,y_mid)
            else:
                painter.drawLine(x1,y1,x1,y_end)
                label=ColorBlock(self.color_sheet.shape_constraints[node.sc_id])
                label.setParent(self)
                label.setAutoFillBackground(True)
                set_geo(label,(x1,y_end),(h,h))
        painter.end()
        self.setPixmap(canvas)
    def set_schema(self,rust_root:RustNode):
        root=Node(rust_root)
        self.root=root
        cal_y(root,1)
        cal_w(root)
        cal_x(root,0)

        self.w,self.h=offset_to_z(root)
        self.h+=SchemaTree.node_h+SchemaTree.interval_y
        self.scale(50)
        self.paint_schema()