from ast import parse,ClassDef,Name,Pass,FunctionDef
from astunparse import unparse

def clean(file_paths: list[str]):
    for file_path in file_paths:
        with open(file=file_path, mode="r") as f:
            code = f.read()
            tree = parse(code)
            for stmt in tree.body:
                if isinstance(stmt,ClassDef):
                    class_def:ClassDef=stmt
                    for dec in class_def.decorator_list:
                        if isinstance(dec,Name):
                            name:Name=dec
                            if name.id=="schema":
                                class_def.body=[Pass()]
                                class_def.decorator_list.remove(dec)
                            elif name.id=="data_operator":
                                class_def.decorator_list.remove(dec)
                                for stmt in class_def.body:
                                    if isinstance(stmt,FunctionDef):
                                        func:FunctionDef=stmt
                                        if func.name=="process":
                                            for arg in func.args.args:
                                                arg.annotation=None


            code = unparse(tree)
        with open(file=file_path, mode="w") as f:
            f.write(code)


# file_paths:Vec<PathBuf>
clean(file_paths)
