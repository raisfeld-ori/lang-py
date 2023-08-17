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
        result = lang_py.actions.async_parse_file(data, "example")


        console.graceful_exit()
    except Exception as error:
        console.panic(error, with_traceback=True)
