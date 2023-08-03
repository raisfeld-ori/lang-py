from python_src import cli, rust_wrapper
from compiler import compiler
import sys

if __name__ == '__main__':
    console = cli.Console(sys.argv)
    try:
        data = console.open_file()
        result = rust_wrapper.Output(compiler.parse.initial_parse(data))
        console.graceful_exit(0)
    except Exception as error:
        console.panic(error, with_traceback=True)
