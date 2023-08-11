from python_src import cli
from python_src.rust_wrapper import *
from lang_py import lang_py
import sys

if __name__ == '__main__':
    console = cli.Console(sys.argv)
    console.debug(
        print_log_errs=False,
        print_too_large=True,
    )
    try:
        data = console.open_file()
        result = handle_output(compiler.parse.initial_parse(data))
        methods = compiler.parse.get_methods(list(map(lambda tup: tup[0], result.statements)))
        methods = map(lambda meth: Method(meth), methods)
        for method in methods:
            console.log(method.input)
            console.window.input()
            console.window.cls()

        console.graceful_exit()
    except Exception as error:
        console.panic(error, with_traceback=True)
