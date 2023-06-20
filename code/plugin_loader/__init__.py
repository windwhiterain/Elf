from code.common import *
def download(git:str):
    """
    download a plugin from git to Elf/plugin,then add a download.json in it,recorde it's git link to find it later.
    :param git: the git link to download the plugin
    :return:
    """
def reload(path:str):
    """
    load a plugin_loader with its relative path in Elf/plugin,override the loaded one with same path if exist
    :param path: the plugin_loader's relative path in Elf/plugin
    :return:
    """
    #TODO
    pass
def load_all():
    """
    load all plugins in Elf/plugin_loader
    :return:
    """
    #TODO
    pass

def find_operator(name:str)->list[Operator]:
    """
    find loaded operators with specific name
    :param name: the name of the operator
    :return: all loaded operator with the name
    """
    #TODO
