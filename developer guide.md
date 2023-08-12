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
```mermaid
graph LR
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
- [Schema](core/src/common/schema.rs)
- [Operator](core/src/common/operator/)
- [Parser](core/src/backend/mod.rs)

