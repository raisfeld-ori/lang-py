from python_src import cli
from python_src.rust_handler import *
import sys

if __name__ == '__main__':
    console = cli.Console(sys.argv)
    try:
        data = console.open_file()
        result = compiler.parse.initial_parse(data)
        console.log()
        console.graceful_exit(0)
    except Exception as error:
        console.panic(error, with_traceback=True)