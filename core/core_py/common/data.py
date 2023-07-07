from typing import *


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

        def __init__(self, dtype: type, dim:int):
            self.dtype = dtype
            self.dim = dim
            self.shape_constraint:Optional[Ref[list[int]]]=None
    def __init__(self, descriptor:Descriptor, value:Any):
        self.descriptor=descriptor
        self.value=value