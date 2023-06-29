import taichi
from typing import Any,Optional

class Data:
    """
    The reference to the actual python data with its description.

    Attributes:
        descriptor:the description of the python data.
        value:the python data.
    """
    class Descriptor:
        """
        The description of a data.

        Attributes:
            dtype:the python type of elements of the data.
            dim:dimension of the python data if its a field.
            shape:if its a field,None means uncertain yet.
        """

        def __init__(self, dtype: type, shape: Optional[list[int]]):
            self.dtype = dtype
            self.shape = shape
        @property
        def dim(self)->int:
            return len(self.shape)
    def __init__(self, descriptor:Descriptor, value:Any):
        self.descriptor=descriptor
        self.value=value