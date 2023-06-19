# Elf -- A node based operator network based on taichi-lang
We have the following features:
1. parallel process:with the powerful taichi-lang,we don't have to
write low level compute shader to run parallel algorithm on GPU.
2. dependency graph:like Unreal,Houdini,Blender,we use a dependency
graph to organize an algorithm in higher level,which brings much 
more flexibility and intuitiveness.However,we have made some changes
to improve some aspects.
3. Embed into other Engine by transferring data via socket.This 
overhead may limit the application in per-frame real-time affairs. 
## Application
### Embed Into Unity Or Other Python Program
We have a socket program to handle data transfer and action 
invoke affairs written in both C# and python,giving chance to
embe Elf in Unity.

Normally we hope to use Elf in the field of computer graphics,
like geometry processing,physical simulation,ray tracing rendering,
loading some data from Engine,use an algorithm to process it them
send back.

However,our design is based on general purpose.So Elf may be used
to machine learning,embed into a python program with or without
using the socket program as it is written in python.
## Network
### Basic Node
```mermaid
graph TB
    A[InputData1]---->J(( ))-->F(duplicate)
    B(( ))-->M[OutputData1]
    F-->G(( ))
    Q[InputData3]---->R(( ))
    subgraph g1[process]
        D[Operator]---E(interface)---B
        D[Operator]---S(interface)---R
    end
    F-->B
    subgraph g2[process]
        H[Operator]---T(interface)---O
        H[Operator]---I(interface)---G
    end
    N[InputData2]--->O(( ))-->P[OutputData2]
    subgraph g3[process]
        K[Operator]---L(interface)---J
    end
```
1. InputData:data in particular access schema while memory schema could vary.
2. Operator:access and even modify data in particular access schema.
3. interface:edit schema on data flow,add or delete,and make sense of the added.
4. duplicate:when accessed by multiple operator,data need to be duplicated,or previous access would affect the latter.However,read only access would not,so there won't be duplication for that.
5. OutputData:data in particular access schema that need to be filled.
### Data flow
```mermaid
graph TB
    in[InputData]--main data flow---d1(( ))---d2(( ))-->out[OutputData]
    d1--branch data flow-->i1[Interface]-->op1[Operator]
    d2--branch data flow-->i2[Interface]-->op2[Operator]
    d3(( ))-.means.-d4((branch))
```
1. Both of the data flow can be sent to only one node with compatible access schema then end.
2. main data flow:from where branch data flow could branch.The order of branch indicates the order of process.
3. branch data flow:could not branch.
4. As a result,there only exist one main data flow to determine the only process order.
## Data Schema
### Access Schema
```mermaid
graph TB
    in[InputData]
    in2[InputData]
    out[OutputData]
    out2[OutputData]
    int(interface)
    int2(interface)
    op[Operator]
    o1(( ))
    o2(( ))
    in--schema1,schema2--- int--schema1,schema2,schema3--- o1---->out
    o2 & o1--choose compatible schema3-->op
    in2--schema3,schema4,schema5--- int2--schema3--- o2---->out2
```
Data flow can have multiple access schema on it.When a operator
perform process on a data flow,it has to choose a compatible
schema to perform on.Access schema is a subset of the original data
so it limits operators and made it possible for one operator to
process on multiple differently structured data flow in the same
access schema.
```mermaid
graph TB
    subgraph g1[reference]
        n12[Compound]
    end
    n10[Schema]-->n11[Primitive] & n12
    n0[Schema]-->n1[Primitive] & n2[Compound]
    subgraph g2[reference]
        subgraph g3[reference]
            n4[Compound]
        end
        n2-->n3[Primitive] & n4
    end
    n9[dimension]-.-n6[taichi.field]
    n8[type]---n6 & n7[variable]-.->n5[primitive]
```
Data in schema are made into multiple data port to be accessed,
which is structured like:
1. Primitive:accessible port in field or variable form of a certain type like int,float3,etc.The access schema can specify field primitive's dim or not,which means any operator processing in such schema should be independent of size on each dimension or even dimension.However,field's shape is changing all the time while accessible,so schema should never specify it.
2. Compound:a namespace for its sub-primitives and sub-compounds.Apart from the name,it should be a reference to another defined access schema.
```mermaid
graph TB
    n10[Schema]-->n11[Primitive] & n12[Primitive] 
    n11 & n12-->n13[ShapeConstrain]
    n0[Schema]-->n1[Primitive] & n2[Compound]
    subgraph g2[reference]
        n2-->n3[Primitive] & n4[Primitive] & n14[ShapeConstrain]
    end
    n1 & n14-->n15[ShapeConstrain]
    n16[ShapeConstrain]-->n17(allocation) & n18(index type)
```
3. ShapeConstrain:some field primitives are logically struct
of array that different components of a struct are stored in
separate field primitives,which means they share a same shape
and same domain of index.Group some primitives and defined shape
constrain into a new shape constrain,which provide memory
allocation method and a specific index type only allowed to use in
constrain members.
### Interface:Add Schema To Data Flow
Besides the schema exists since the data flow was created,interface
can add a new one to the data flow,by making references to the
data port of existing schema. 
```mermaid
graph LR
    io1[input data flow] 
    io2[output data flow]
    subgraph interface
         subgraph g0[exist schema]
            s0[Schema]---p1[Primitive] 
         end
         subgraph g1[exist schema]
            s1[Schema]--- c1[compound]---p2[Primitive] & p6[Primitive]
         end
         subgraph g2[new schema]
            p4[Primitive] & c2[Compound]---s2[Schema]
            p5[Primitive] ---c2
         end
         p2-.-l(connection)-.-p4
         p1-.-p5
    end
    io1-->s1 & s0
    s2-->io2
    io1-->io2
```
```mermaid
graph LR
    io1[input data flow] 
    io2[output data flow]
    subgraph interface
         subgraph g1[exist schema]
            s1[Schema]---c3[Compound] & c1[Compound] 
            c1---p2[Primitive] & p1[Primitive] 
         end
         subgraph g2[new schema]
            c4[Compound] & c2[Compound]---s2[Schema]
            p5[Primitive] & p6[Primitive]---c2
         end
         p2-.-p5
         p1-.-p6
         c3-.-c4
         c1-.equal to.->c3
    end
    io1-->s1
    s2-->io2
    io1-->io2
```
1. connection:define a reference from one data port of the new schema
to a exists schema's data port.It can be performed
on two primitives or two Compound in compatible schema.Performing
on Compound is equal to performing on every component of it.
2. The output data flow is not a duplication but a
reference to the existed Data's subset.
3. The type of data flow won't change from 
input to output.Branch data flow use this to fit the access schema
of the operator it is sent to;main data flow use this to change access
schema for next part of the network. 
## Duplicate
```mermaid
graph TB
    in[InputData]--main data flow-->du(duplicate)--main data flow-->op1[Operation A] & op2[Operation B]
```
1. Output multiple main data flow,each one is independent of others.
```mermaid
graph TB
    subgraph Data
        p1(port)
        p2(port)
        p3(port)
    end
    f1[data flow]
    f2[data flow]
    f3[data flow]
    p1-->r1[read]---f1 & f2
    p2-->w1[write ^n]
    w1---f2
    w1-.-f1
    p3-->r2[read] & w2[write ^1]
    r2---f1
    w2---f3
```
2. To figure out the way with minimum duplications of port data,collect the following read and write on port for each data flow
3. For each port,according to the number of data flow that read and write on it,decide whether a duplication is needed.
## Network Folding
Network can be folded as operator.
```mermaid
graph TB
    subgraph g1[network to fold]
        in1[InputData]
        out1[OutputData]
        in1--main data flow-->out1
    end
    in2[InputData]
    out2[OutputData]
    in2--- o1(( ))--main data flow-->out2
    subgraph g3[reference to the folded]
        op1[new Operator]
    end
    o1--branch data flow-->op1
```
```mermaid
graph TB
    subgraph g1[network to fold]
        in1[InputData]
        out1[OutputData]
        int1(interface)
        sc1(Schemas)
        sc2(schemas)
        in1--- sc1--- int1--- sc2 -->out1
    end
    in2[InputData]
    out2[OutputData]
    sc3(Schemas)
    sc4(schemas)
    subgraph g3[reference to the folded]
        op1[new Operator]
    end
    sc1-.-sc3
    sc2-.-sc4
    in2--- sc3--- op1--- sc4-->out2
```
1. The new operator's access schema is the same with the schema
of input data node in the network.
2. The data flow pulled out of the input data node in the network
is always main data flow no matter what the new operator receives.
The process order is to process the network inside an operator then
next operator.
3. If a branch data flow is sent to the new operator,the flow
end,and the operator has no output.However,the output data node
in the network is still make sense by acting as the starting 
point for solving the dependency graph indicated by the network.
4. If a main data flow is sent to the new operator,the operator
generate a main data flow with schemas indicated by the output
data node of the network.
5. From this point of view,interface node is a kind of operator.
## Plugin
In Elf,a project is a plugin.
### Directory Structure
- \<plugin name>
  - infor.json
    ```json
      {
        "dependency": [
          {"URL": "a valid URL of another plugin_loader"}
        ],
        "version":[0,0,1],
        "name": "template",
        "description": "template plugin for instruction",
      }
    ```
  - code
    - main.py
      ```python
        import meta as elf
        @elf.schema
        class Ray:
          start:elf.vec3[1]#field primitive,dimension is 1
          direction:elf.vec3[1]
          all_sc=elf.ShapeConstrain(start,direction)
        @elf.schema
        class Light:
          ray:Ray#compound:refers to Ray
          energe:float[1]
          line_sc=elf.ShapeConstrain(ray.all_sc,energe)
          density:float[3]#dimension is 3
          mode:int#variable primitive
        @elf.operator
        class MoveLight(elf.Operator):
          def process(self,light:Light):#entry:parameter type determines the schema
              for index in elf.ndrange(light.line_sc.shape):#get shape from ShapeConstrain
                  start=light.ray.start[index]
                  direction=light.ray.direction[index]
                  energe=light.energe[index]
                  light.ray.start[index]+=direction*energe*light.density[round(direction)]*light.mode
      ```
  - network
    - \<network name>
1. ./infor.json:description of the plugin.
2. ./code:define schema,write code for primitive operator.
3. ./code/main.py:an code example.
4. ./network:store network directory build by Elf.
5. ./network/\<network name>:an example network that inner content
should only be accessed via Elf. 
6. When editing a plugin's network in Elf,schemas and operators
defined in the same plugin or its dependent plugin is available.
same plugin 
### Compile To Taichi
```python
  #exists else where
  @taichi.data_oriented
  class ShapeConstrain:
      def __init__(self,shape:tuple[int]):
          self.shape=shape
  #compile result
  @taichi.data_oriented
  class Ray:
      def __init__(self,all_sc:tuple[int]):
          self.start=taichi.field(vec3,shape=all_sc)
          self.direction=taichi.field(vec3,shape=all_sc)
          self.all_sc=ShapeConstrain(all_sc)
  @taichi.data_oriented
  class Light:
      def __init__(self,line_sc:tuple[int],density:tuple[int]):
          self.ray=Ray(line_sc)
          self.energe=taichi.field(float,shape=line_sc)
          self.line_sc=ShapeConstrain(line_sc)
          self.density=taichi.field(float,shape=density)
          self.mode=int()
  @taichi.data_oriented
  class MoveLight:
      @taichi.kernel
      def process(self,light:taichi.template()):
          for index in taichi.ndrange(light.line_sc.shape):
              start=light.ray.start[index]
              direction=light.ray.direction[index]
              energe=light.energe[index]
              light.ray.start[index]+=direction*energe*light.density[round(direction)]*light.mode 
```
## Advantages Over The Node System Of Houdini
1. Data are typed by schema.While houdini has no type system.
2. Interface gives an efficient way for operator to
process data in different schema.While houdini have to specify
the way operator interpret the data, on operator itself.
3. Duplication is minimized.While houdini have cache on nealy 
every node.
4. Operator can be viewed as an input to another operator through
branch data flow,with the help of access schema.Which can hardly
be done in houdini because of its purely functional semantics.





