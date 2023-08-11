import importlib.util


class Ref:
    def __init__(self, value):
        self.value = value


class ChainRef:
    def __init__(self, is_end: bool, value):
        self.is_end = is_end
        self.value = value

    def get_end(self):
        ret = self
        while not ret.is_end:
            ret = ret.value
        return ret.value


def import_module_by_path(module_path):
    """
    根据给定的完整路径动态导入模块
    """
    spec = importlib.util.spec_from_file_location("module_name", module_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
