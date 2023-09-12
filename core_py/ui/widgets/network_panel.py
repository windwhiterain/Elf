from typing import Optional
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

class Connection():
    def __init__(self,port1:'NodePort',port2:'NodePort'):
        if port1.type==NodePort.Type.input and port2.type==NodePort.Type.output:
            self.output=port2
            self.input=port1
        elif port2.type==NodePort.Type.input and port1.type==NodePort.Type.output:
            self.output=port1
            self.input=port2
        else:raise Exception()
        self.output.connections.append(self)
        self.input.connections.append(self)
    def deconnect(self):
        self.output.connections.remove(self)
        self.input.connections.remove(self)
    def output_center(self)->tuple[int,int]:
        return self.output.in_panel_center()
    def input_center(self)->tuple[int,int]:
        return self.input.in_panel_center()
    def another(self,port)->'NodePort':
        if port is self.input:return self.output
        elif port is self.output:return self.input
        else:raise Exception()
    def is_ports(self,port1:'NodePort',port2:'NodePort')->bool:
        return (port1 is self.input and port2 is self.output)\
            or (port2 is self.input and port1 is self.output)
class NodePort(QLabel):
    class Type:
        input=0
        output=1
    def __init__(self,node:'Node',type:Type):
        super().__init__()
        self.node=node
        self.type=type
        self.connections=list[Connection]()
        self.setStyleSheet(self.node.qss)
        self.setParent(node)
        self.show()
    def mousePressEvent(self, e):
        selecteds=self.node.panel.selected_ports
        if len(selecteds)==0:
            self.select()
        else:
            for selected in selecteds:
                self.node.panel.add_connection(selected,self)
            while len(selecteds)!=0:
                selecteds[len(selecteds)-1].unselet()
    def select(self):
        self.node.panel.selected_ports.append(self)
        self.setStyleSheet(self.node.selected_qss)
    def unselet(self):
        self.node.panel.selected_ports.remove(self)
        self.setStyleSheet(self.node.qss)
    def in_panel_center(self)->tuple[int,int]:
        return add(tuplize(self.node.pos()),center(self))
class NodeMain(QLabel):
    def __init__(self,node:'Node'):
        super().__init__()
        self.node=node
        self.setParent(node)
        self.show()
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
            self.node.drag(e)
        if self.node.on_port:
            self.node.scale_port_count(e)
class Node(QWidget):
    def __init__(self,pos:tuple[float,float],node_type:NodeType,index:int,panel:'NetworkPanel'):
        super().__init__()
        set_center(self,pos)
        self.input_ports=list[NodePort]()
        self.output_ports=list[NodePort]()
        self.on_drag=False
        self.on_port=False
        self.index=index
        self.node_main=NodeMain(self)
        self.color_sheet=node
        self.panel=panel
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
        self.selected_qss="""
            QLabel{
                background-color:"""+self.color_sheet.bg.get().name()+""";
                border-style:solid;
                border-width:2px;
                border-color:"""+brighten(frame_color.get()).name()+""";
            }
        """
        self.setStyleSheet(self.qss)
        self.refresh(1,1)
    def drag(self,e):
        d_mouse_pos=sub(tuplize(e.globalPos()),self.start_mouse_pos)
        goal_pos=add(self.start_center,d_mouse_pos)
        self.panel.move_node(self,goal_pos)
    def scale_port_count(self,e):
        '''change port's count action'''
        d_x=e.globalPos().x()-self.start_mouse_pos[0]
        d_count_in=round(d_x/20)
        goal_count_in=max(self.start_input_port_count+d_count_in,1)
        d_y=e.globalPos().y()-self.start_mouse_pos[1]
        d_count_out=round(d_y/20)
        goal_count_out=max(self.start_output_port_count+d_count_out,1)
        self.refresh(goal_count_in,goal_count_out)
    def refresh(self,input_port_count:int,output_port_count:int):
        '''update port's ui and node size'''
        gen_input_port=lambda :NodePort(self,NodePort.Type.input)
        gen_output_port=lambda :NodePort(self,NodePort.Type.output)
        des_port=lambda port:(port.deleteLater())
        def destructible(port:NodePort)->bool:
            return len(port.connections)==0
        input_port_count=resize(self.input_ports,input_port_count,gen_input_port,des_port,destructible)
        output_port_count=resize(self.output_ports,output_port_count,gen_output_port,des_port,destructible)
        count=max(input_port_count,output_port_count)
        width=60+count*30
        set_size(self,(width,50))
        set_geof(self.node_main,(self.width()/2,self.height()/2),(self.width(),30))
        for index,port in enumerate(self.input_ports):
            set_geof(port,((index+1)/(input_port_count+1)*self.width(),5),(8,8))
        for index,port in enumerate(self.output_ports):
            set_geof(port,((index+1)/(output_port_count+1)*self.width(),self.height()-5),(8,8))
        self.panel.update()
        

class NetworkPanel(QWidget):
    def __init__(self):
        super().__init__()
        self.nodes=list[Node]()
        self.selected_ports=list[NodePort]()
        self.connections=list[Connection]()
        self.choose_list=ChooseList(self.find_add_node)
        self.choose_list.setParent(self)
        self.color_sheet=network_panel
    def paintEvent(self,e):
        painter=QtGui.QPainter(self)

        brush = QtGui.QBrush()
        brush.setColor(self.color_sheet.bg.get())
        brush.setStyle(Qt.SolidPattern)
        rect = QtCore.QRect(0, 0, painter.device().width(), painter.device().height())
        painter.fillRect(rect, brush)   

        pen = painter.pen()
        pen.setColor(QColor(255, 255, 255))
        pen.setWidth(3)
        painter.setPen(pen)
        for connection in self.connections:
            painter.drawLine(qpointlize(connection.output_center()),qpointlize(connection.input_center()))
        painter.end()
    def clear(self):
        '''clear all node ui'''
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
        '''find node by choose_list's text then add it'''
        text=self.choose_list.text()
        ids=context.find_nodes(text)
        if len(ids)>0:
            self.add_node(self.event_pos,NodeType.DataOperator,ids[0])
    def add_node(self,pos:tuple[float,float],node_type:NodeType,id:int):
        self.network.nodes.append(elf_rust.Node(node_type,id,pos))
        self.add_node_ui(pos,node_type,len(self.network.nodes)-1)
    def add_node_ui(self,pos:tuple[float,float],node_type:NodeType,id:int):
        node=Node(pos,node_type,id,self)
        self.nodes.append(node)
        node.setParent(self)
        node.show()
    def find_connection(self,port1:NodePort,port2:NodePort)->Optional[Connection]:
        for connection in self.connections:
            if connection.is_ports(port1,port2):
                return connection
        return None
    def add_connection(self,port1:NodePort,port2:NodePort):
        connection=self.find_connection(port1,port2)
        if connection is None:
            self.connections.append(Connection(port1,port2))
        else:
            self.connections.remove(connection)
            connection.deconnect()
        self.update()
    def move_node(self,node:Node,center:tuple[float,float]):
        set_center(node,center)
        self.update()


        
        
