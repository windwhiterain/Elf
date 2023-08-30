from json import load
import os
import elf_rust
file_path = os.path.dirname(__file__)
settings = load(open(file_path+"/../settings.json"))
env_path = settings["python_interpreter_path"]
plugin_path = file_path+"/../plugin"

context = elf_rust.Context(env_path, plugin_path)
context.load_resource()