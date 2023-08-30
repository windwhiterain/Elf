import PySide6.QtGui
from ui.widgets.color_block import *
from PySide6 import QtWidgets, QtGui
from ui.help import *
from ui.palette import network_panel
from ui.palette import node
from ui.palette import ColorLink
from PySide6.QtWidgets import QSizePolicy
from PySide6.QtCore import Qt
from ui.widgets.choose_list import ChooseList
from context import context
from enum import Enum
import elf_rust
from elf_rust import NodeType

class NodePort(QLabel):
    def __init__(self,node:'Node'):
        super().__init__()
        self.node=node
        self.setStyleSheet(node.qss)
        self.setParent(node)
        self.show()
class NodeMain(QLabel):
    def __init__(self,node:'Node'):
        super().__init__()
        self.node=node
        self.setParent(node)
    def mousePressEvent(self, e):
        self.node.start_mouse_pos=tuplize(e.globalPos())
        match e.button():
            case Qt.MouseButton.LeftButton:
                self.node.on_drag=True
                self.node.start_center=center(self.node)
            case Qt.MouseButton.RightButton:
                self.node.on_port=True
                self.node.start_input_port_count=len(self.node.input_ports)
                self.node.start_output_port_count=len(self.node.output_ports)
    def mouseReleaseEvent(self, e):
        match e.button():
            case Qt.MouseButton.LeftButton:
                self.node.on_drag=False
            case Qt.MouseButton.RightButton:
                self.node.on_port=False
    def mouseMoveEvent(self, e):
        if self.node.on_drag:
            self.node.move_with_mouse(e)
        if self.node.on_port:
            self.node.scale_port_count(e)
        
class Node(QWidget):
    def __init__(self,pos:tuple[float,float],node_type:NodeType,index:int):
        super().__init__()
        set_center(self,pos)
        self.input_ports=list[NodePort]()
        self.output_ports=list[NodePort]()
        self.on_drag=False
        self.on_port=False
        self.index=index
        self.node_main=NodeMain(self)
        self.color_sheet=node
        match node_type:
            case NodeType.DataOperator:
                frame_color=self.color_sheet.frame_data_operator
            case NodeType.InterfaceOperator:
                frame_color=self.color_sheet.frame_interface_operator
        self.qss="""
            QLabel{
                background-color:"""+self.color_sheet.bg.get().name()+""";
                border-style:solid;
                border-width:2px;
                border-color:"""+frame_color.get().name()+""";
            }
        """
        self.setStyleSheet(self.qss)
        self.refresh(1,1)
        self.refresh(5,5)
        self.refresh(1,1)
    def move_with_mouse(self,e):
        d_mouse_pos=sub(tuplize(e.globalPos()),self.start_mouse_pos)
        goal_pos=add(self.start_center,d_mouse_pos)
        set_center(self,goal_pos)
    def scale_port_count(self,e):
        d_x=e.globalPos().x()-self.start_mouse_pos[0]
        d_count_in=round(d_x/20)
        goal_count_in=max(self.start_input_port_count+d_count_in,1)
        d_y=e.globalPos().y()-self.start_mouse_pos[1]
        d_count_out=round(d_y/20)
        goal_count_out=max(self.start_output_port_count+d_count_out,1)
        self.refresh(goal_count_in,goal_count_out)

    def refresh(self,input_port_count:int,output_port_count:int):
        count=max(input_port_count,output_port_count)
        width=60+count*30
        set_size(self,(width,50))
        set_geof(self.node_main,(self.width()/2,self.height()/2),(self.width(),30))
        gen_port=lambda :NodePort(self)
        des_port=lambda port:(port.deleteLater())
        assert(self.input_ports is not None)
        resize(self.input_ports,input_port_count,gen_port,des_port)
        resize(self.output_ports,output_port_count,gen_port,des_port)
        assert(self.input_ports is not None)
        
        print(input_port_count)
        for index,port in enumerate(self.input_ports):
            set_geof(port,((index+1)/(input_port_count+1)*self.width(),5),(8,8))
        for index,port in enumerate(self.output_ports):
            set_geof(port,((index+1)/(output_port_count+1)*self.width(),self.height()-5),(8,8))
        

class NetworkPanel(QWidget):
    def __init__(self):
        super().__init__()
        self.nodes=list[Node]()
        self.choose_list=ChooseList(self.find_add_node)
        self.choose_list.setParent(self)
        self.color_sheet=network_panel
        self.setStyleSheet("""
            QWidget{
                background-color:"""+self.color_sheet.bg.get().name()+"""
            }
        """)
    def clear(self):
        for node in self.nodes:
            node.destroy()
        self.nodes.clear()
    def refresh(self,network):
        self.network=network
        self.clear()
        for node in network.nodes:
            self.add_node_ui(node.pos)
    def mousePressEvent(self, event):
        if event.button()==Qt.MouseButton.RightButton:
            self.event_pos=(event.localPos().x(),event.localPos().y())
            self.choose_list.open(self.event_pos)
            
    def find_add_node(self):
        text=self.choose_list.text()
        ids=context.find_nodes(text)
        if len(ids)>0:
            self.add_node(self.event_pos,NodeType.DataOperator,ids[0])
    def add_node(self,pos:tuple[float,float],node_type:NodeType,id:int):
        self.network.nodes.append(elf_rust.Node(node_type,id,pos))
        self.add_node_ui(pos,node_type,len(self.network.nodes)-1)
    def add_node_ui(self,pos:tuple[float,float],node_type:NodeType,id:int):
        node=Node(pos,node_type,id)
        self.nodes.append(node)
        node.setParent(self)
        node.show()


        
        
