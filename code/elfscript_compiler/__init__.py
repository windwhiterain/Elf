from code.common import *
import ast
import code.elfscript_compiler.find_decorated


def compile(code:str)->tuple[list[type[Schema]],list[type[Operator]]]:
    """
    use ast to modify the code in elfscript and extract schemas and operators defined in it
    :param code: user written elfscript in a plugin
    :return: schemas and operators defined in code
    """
    module:ast.Module=ast.parse(code)
    find_decorated.find_decorated(module)
    #TODO
