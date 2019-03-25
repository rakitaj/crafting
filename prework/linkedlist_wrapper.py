import ctypes

class Node(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.c_int),
        ("next", ctypes.pointer)
    ]