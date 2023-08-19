import subprocess
import os
from json import load
file_path = os.path.dirname(__file__)
settings = load(open(file_path+"/../../settings.json"))
env_path = settings["python_interpreter_path"]


def command(command: list[str]):
    res = subprocess.run(command, capture_output=True,
                         text=True, shell=True, cwd=file_path+"/../../core/")
    print("command output:", res.stdout, res.stderr)
    if res.returncode != 0:
        raise Exception()


def compile():
    command(["maturin", "build", "-i", "python"])
    command(["pip", "install", "--force-reinstall", "--upgrade",
             "-t", env_path+"/Lib/site-packages", "./target/wheels/elf-0.1.0-cp310-none-win_amd64.whl"])


def run():
    command(["call", env_path+"/Scripts/activate", "\n",
            "python", "../core_py/user_starter/__init__.py"])
