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
        result = handle_output(lang_py.parse.initial_parse(data))
        methods = list(map(lambda method: Method(method),lang_py.parse.get_base_methods(result.statements, result.shallow_code)))
        for method in methods:
            for line in method.lines:
                console.log(method.name, " ",line.actual_line(), " ", line.position())
                console.window.input()
                console.window.cls()
            else:
                console.log("done")
        console.graceful_exit()
    except Exception as error:
        console.panic(error, with_traceback=True)
