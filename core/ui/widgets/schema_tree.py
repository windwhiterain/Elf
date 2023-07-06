from core.ui.common import *
from core.ui.widgets.color_block import *
class SchemaTree(QtWidgets.QLabel):
    interval_x=1.0
    interval_y=0.3
    node_w=2.0
    node_h=0.5
    class ColorSheet:
        def __init__(self,bg:Color,line:Color,shape_constraints:list[Color]):
            self.bg=ColorLink(bg)
            self.line=ColorLink(line)
            self.shape_constraints=[ColorLink(color) for color in shape_constraints]
    class Node:
        def __init__(self,name:str,childs:list['SchemaTree.Node'],sc_id:int):
            self.x,self.y,self.w=0.0,0.0,0.0
            self.name=name
            self.childs=childs
            self.sc_id=sc_id
        def cal_y(self,y:float):
            self.y=y
            for child in self.childs:
                child.cal_y(y+SchemaTree.interval_y+SchemaTree.node_h)
        def cal_w(self):
            self.w-=SchemaTree.interval_x
            for child in self.childs:
                self.w +=SchemaTree.interval_x
                child.cal_w()
                self.w+=child.w
            if self.w<SchemaTree.node_w:
                self.w=SchemaTree.node_w
        def cal_x(self,x:float):
            self.x=x
            c_x=self.x-self.w/2
            for child in self.childs:
                c_x+=child.w/2
                child.cal_x(c_x)
                c_x+=child.w/2
                c_x+=SchemaTree.interval_x
        def iter(self)->Iterable['SchemaTree.Node']:
            yield self
            for child in self.childs:
                for ret in child.iter():
                    yield ret
        def cal_edge(self)->tuple[float,float,float,float]:
            min_x,min_y,max_x,max_y=math.inf,math.inf,-math.inf,-math.inf
            for node in self.iter():
                if node.x>max_x:max_x=node.x
                if node.x<min_x:min_x=node.x
                if node.y>max_y:max_y=node.y
                if node.y<min_y:min_y=node.y
            min_x-=SchemaTree.node_w/2
            max_x+=SchemaTree.node_w/2
            min_y-=SchemaTree.node_h/2
            max_y+=SchemaTree.node_h/2
            return (min_x,min_y,max_x,max_y)
        def offset_to_z(self)->tuple[float,float]:
            min_x, min_y, max_x, max_y=self.cal_edge()
            for node in self.iter():
                node.x-=min_x
                node.y-=min_y
            return max_x-min_x,max_y-min_y
    def __init__(self,color_sheet:'SchemaTree.ColorSheet'):
        super().__init__()
        self.root:Optional['SchemaTree.Node']=None
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
        for node in self.root.iter():
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
    def set_schema(self,root:Node):
        self.root=root
        root.cal_y(0)
        root.cal_w()
        root.cal_x(0)
        self.w,self.h=root.offset_to_z()
        self.h+=SchemaTree.node_h+SchemaTree.interval_y
        for node in self.root.iter():
            print(node.x)
        self.scale(50)
        self.paint_schema()