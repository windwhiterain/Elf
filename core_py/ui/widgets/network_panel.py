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

class Node(QLabel):
    def __init__(self,pos:tuple[float,float],node_type:NodeType,index:int):
        super().__init__()
        set_geof(self,pos,(90,30))
        self.index=index
        self.color_sheet=node
        match node_type:
            case NodeType.DataOperator:
                frame_color=self.color_sheet.frame_data_operator
            case NodeType.InterfaceOperator:
                frame_color=self.color_sheet.frame_interface_operator
        self.setStyleSheet("""
            QLabel{
                background-color:"""+self.color_sheet.bg.get().name()+""";
                border-style:solid;
                border-width:2px;
                border-color:"""+frame_color.get().name()+""";
            }
        """)

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


        
        
