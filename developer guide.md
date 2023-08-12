# Developer Guide
## Pipeline
```mermaid
graph TD
    subgraph backend[Backend]
        plugins[Plugins]
        target[Target Program]
    end
    plugins--load-->resource[Resource]-.refered by.->network[Network] & ngraph[Graph]
    network--compile to-->ngraph
    ngraph--generate-->target
    
```
## Backend
### Load
```mermaid
graph TB
    meta[global context]
    subgraph pg[plugin folder]
        subgraph es[host script program]
            subgraph ecope[elf scope]
                sm[schema sytex]
                op[operator sytex]
            end
            pycope[host script scope]
        end
        subgraph fn[serialized folder]
            network[network file]
        end
    end
    meta--> sm & op
    subgraph ec[Parser]
        smc(parse schema)
        opc(parse operator)
    end
    pgld(serializer)
    sm-->smc-->smd
    op-->opc-->opd
    pycope-.refered by.->opd
    fn---pgld---net
    subgraph resource[Resource]
        net[Network]
        opd[Operator]
        smd[Schema]
    end
```
- [global context]():reserved words in host script which compose the elf scope.
- [Schema](core/src/common/schema.rs):backend irrelevant.
- [Operator](core/src/common/operator/):refers to host script code's location.
- [Parser](core/src/backend/mod.rs)
### Generate
```mermaid
graph LR
    subgraph graph[Graph]
        plugins[dependent plugins]
        datas[datas]
        interfaces[interfaces]
        nodes[nodes]
    end
    subgraph host[host script program]
        runtime[runtime]
        locate[host code entry]
        ref[reference host program] 
        nodes2[nodes]
        context[context]
        structs[struct definition]
    end
    subgraph resource[resource]
        Schema
        Operator
    end
    plugins-->cleaner([Cleaner])-->ref
    datas & interfaces-->context
    Schema-->structs
    Operator-->locate
    nodes-->nodes2
    plugins-.filter.->Schema & Operator
```
- [Graph](core/src/graph/mod.rs):low level dependency graph.
- [Cleaner(taichi)](core/src/backend/taichi/generator/cleaner/mod.rs):remove elf scope from host program.
### Host Program
```mermaid
graph TD
    runtime[runtime]
    locate[host code entry]
    ref[reference host program] 
    nodes[nodes]
    context[context]
    structs[struct definition]
    runtime-.solve dependency.->nodes-.modify.->context
    nodes-.use.->locate-.refer to.->ref-.modify.->context

```
- [runtime(taichi)](core/src/backend/taichi/generator/__init__.py):dependency graph solver.
- [host code entry]():find function entry in reference host program. 


