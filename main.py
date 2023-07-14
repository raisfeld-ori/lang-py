from rust_header import rust_header
from config import *
from platform import system
import sys
from os import getcwd

if __name__ == '__main__':
    # defaults
    OS = system()
    LANG = language.rust

    # if the file is being ran from the python interpreter, then the file is argv[1]
    if (sys.argv[0] == __file__ and len(sys.argv) == 1) or len(sys.argv)  == 0:
        file = None
    elif sys.argv[0] == __file__:
        file = sys.argv[1]
    else:
        file = sys.argv[0]

    print(getcwd())