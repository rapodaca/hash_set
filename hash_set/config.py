import sys, ctypes
from ctypes import POINTER, Structure, c_size_t, c_int8, c_bool

class HashSetS(Structure):
    pass

def load_lib():
    prefix = {'win32': ''}.get(sys.platform, 'lib')
    extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
    lib = ctypes.cdll.LoadLibrary(prefix + "hash_set" + extension)

    lib.hash_set_new.restype = POINTER(HashSetS)

    lib.hash_set_len.argstype = (POINTER(HashSetS), POINTER(c_size_t), )
    lib.hash_set_len.restype = c_int8

    lib.hash_set_contains.argstype = (POINTER(HashSetS), POINTER(c_size_t), POINTER(c_bool), )
    lib.hash_set_contains.restype = c_int8

    lib.hash_set_insert.argstype = (POINTER(HashSetS), c_size_t)
    lib.hash_set_insert_restype = c_bool

    lib.hash_set_collect.argstype = (POINTER(HashSetS), POINTER(POINTER(c_size_t)), )

    return lib

lib = load_lib()