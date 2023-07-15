from rust_header import rust_header
from python_src.config import *
import python_src.cli as cli
import sys
from os import getcwd

if __name__ == '__main__':
    args = cli.handle_args(sys.argv)
