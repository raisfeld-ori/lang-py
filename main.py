from rust_header import rust_header
from python_src.config import *
import python_src.args as args
from platform import system
import sys
from os import getcwd
print(sys.argv)
if __name__ == '__main__':
    # defaults
    OS = system()
    LANG = language.rust

