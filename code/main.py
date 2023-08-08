from python_src import cli, rust_wrapper
from compiler import compiler
import sys

if __name__ == '__main__':
    console = cli.Console(sys.argv)
    console.debug(
        print_log_errs=False,
        print_too_large=True,
    )
    try:
        data = console.open_file()
        result = rust_wrapper.handle_output(compiler.parse.initial_parse(data))
        console.graceful_exit()
    except Exception as error:
        console.panic(error, with_traceback=True)
