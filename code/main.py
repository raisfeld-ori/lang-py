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
        show_result = console.select("parsed initial result, show it?", ["yes", "no"])
        if show_result == 0:
            console.log(f"found:\n"
                        f"{len(result.variables)} variables,\n"
                        f"{len(result.statements)} statements,\n"
                        f"{len(result.executables)}  methods being called,\n"
                        f"{len(result.unknown)} unknown lines of code")
            console.log("press enter to continue")
            console.window.input()
            show_result = console.select("what would you like to handle the unknown lines?", ["yes", "no"])
            if show_result == 0:
                for unknown in result.unknown:
                    console.log(unknown[0].actual_line())
                    console.window.input()
                    console.window.cls()
        console.graceful_exit(0)
    except Exception as error:
        console.panic(error, with_traceback=True)
