from abc import ABC,abstractmethod
from core.common import Context
class Executable(ABC):
    @abstractmethod
    def application_start(self):
        pass
    @abstractmethod
    def application_end(self):
        pass
    @abstractmethod
    def execute(self,**kwargs:Context):
        pass
