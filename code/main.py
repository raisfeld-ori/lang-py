from python_src import cli
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
        result = lang_py.actions.async_get_module(data, "example")
        for line in result.to_pysort():
            console.log(line.shallow_global().actual_line())
            console.window.input()
            console.window.cls()
        console.graceful_exit()
    except Exception as error:
        console.panic(error, with_traceback=True)
