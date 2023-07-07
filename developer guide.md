# Developer Guide
## Pipeline
```mermaid
graph LR
    subgraph pg[Plugin]
        subgraph es[ElfScript]
            subgraph ecope[Elf Scope]
                sm[schema sytex]
                op[operator sytex]
            end
            pycope[Python Scope]
        end
        fn[folded network file]
    end
    subgraph ec[ElfSCript Compiler]
        smc[Schema Compiler]
        opc[Operator Compiler]
    end
    sm-->smc-->con[Context] & smd(schema description)
    op-->opc-->exe[Executable] & opd(operator description)
    pycope-->exe
    pgld[Plugin Loader]
    fn-->pgld-->net[Network]
    netan[Network Analizer]
    smd & opd & net-->netan-->annet[Analized Network]
    exebd[Executable Builder]
    con & exe & annet-->exebd-->exeres[Executable]
```
## Plugin Loader
```mermaid
graph LR
    meta[meta]
    subgraph pg[Plugin]
        subgraph es[ElfScript]
            subgraph ecope[Elf Scope]
                sm[schema sytex]
                op[operator sytex]
            end
            pycope[Python Scope]
        end
        fn[folded network file]
    end
    meta--> sm & op
    subgraph pl[Plugin Loader]
        subgraph ec[ElfSCript Compiler]
            smc(Schema Compiler)
            opc(Operator Compiler)
        end
        pgld(network serializer)
    end
    sm-->smc-->smd[Schema]
    op-->opc-->exe[Executable] & opd[Operator]
    pycope-->exe
    
    fn---pgld---net[Network]
```
- [meta](meta):python code that won't be executed,only import to elfscript
for code hint.
- [Schema](core/core_py/common/schema.py):the description of a schema.
- [executable](core/core_py/common/executable.py):a python class that actually perform the data
modification via context.
- [Operator](core/core_py/common/operator_resource.py):the description of a operator.

