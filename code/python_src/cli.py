"""
cli:
the file that takes care of the cli interface.

it parses any args that is being passes into it,
and handles any errors that come with it.
"""
import curses
import sys
from enum import Enum, auto
import traceback

class repr_methods(Enum):
    repr = auto()
    to_str = auto()
    name = auto()

class cases(Enum):
    Default = """
    the lang-py-compiler.
    use lang-py-compiler --help for more info
    """
    Help = """
    the lang-py-compiler compiles python code into other programming languages code.
    
    how to use the compiler:
        if you want to compile a file, then simply write the file's name,
        using this method, you can get rust code that does the same thing as the original python code.
    
    current kwargs:
        --help -> writes how to use the compiler, and the current existing kwargs
        -h -> writes how to use the compiler, and the current existing kwargs
        -py -> when using the python interpreter, use -py to select the right file
        
    
    WARNING:
        the compiler is in beta, and is bound to have a lot of errors and bugs
    
    """
class Window:
    def __init__(self, std: curses.window, std_color = curses.COLOR_WHITE):
        self.std = std
        self.std.clear()
        self.std.refresh()
        self.current_color = std_color

    def write(self, obj):
        self.std.addstr(obj, self.current_color)

    def edit_color(self, new_color):
        self.current_color = new_color


    def exit(self, state: int = 0):
        self.std.addstr("\npress enter to exit\n")
        self.std.getch()
        sys.exit(state)

class Console:
    def __init__(self, argv: list[str]):
        """
        argv: sys.argv

        parses every arg in argv, and returns the appropriate
        result for every one of them
        """
        curses.initscr()
        self.output = self._handle_args(argv)
        self.window = curses.wrapper(Window)

        curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_YELLOW, curses.COLOR_BLACK)
        curses.init_pair(3, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(4, curses.COLOR_GREEN, curses.COLOR_BLACK)
        self.NORMAL_COLOR = curses.color_pair(1)
        self.WARNING_COLOR = curses.color_pair(2)
        self.ERROR_COLOR = curses.color_pair(3)
        self.SUCCESS_COLOR = curses.color_pair(4)
        self.warnings = 0

    def warn(self, error: Exception | str, with_traceback: bool = False, suggestion: str = None):
        previous_color = self.window.current_color
        self.window.edit_color(self.WARNING_COLOR)
        if with_traceback:
            for line in traceback.extract_stack().format()[:-1]:
                self.log(line, end="")
        if type(error) == str:
            self.log(error)
        else:
            self.log(error.__class__.__name__, ": ", error.args[0])
        if suggestion:
            self.log(suggestion)


        self.window.edit_color(previous_color)
        self.warnings += 1


    def panic(self, error: Exception | str, with_traceback: bool = False, suggestion: str = None):
        self.window.edit_color(self.ERROR_COLOR)
        if with_traceback:
            for line in traceback.extract_stack().format()[:-1]:
                self.log(line, end="")
        if type(error) == str:
            self.log(error)
        else:
            self.log(error.__class__.__name__, ": ", error.args[0])
        self.window.edit_color(self.SUCCESS_COLOR)
        if suggestion:
            self.log("suggestion -> ",suggestion)
        self.window.exit(1)

    def log(self, *args, end: str = "\n", method: repr_methods = repr_methods.repr ) -> None:
        try:
            for arg in args:
                if type(arg) == str:
                    self.window.write(arg)
                    continue

                match method:
                    case repr_methods.repr:
                        self.window.write(f"{arg}")
                    case repr_methods.to_str:
                        self.window.write(str(arg))
                    case repr_methods.name:
                        self.window.write(arg.__name__)

            self.window.write(end)
        except Exception as error:
            self.panic(error, with_traceback=True, suggestion="try using another repr methods from python_src.enums")
    def graceful_exit(self, state: int = 0):
        self.window.edit_color(self.SUCCESS_COLOR)
        self.log("warnings: ", self.warnings)

        self.window.exit(state)


    def _handle_args(self, argv: list[str]) -> str:
        """
        handle args is just
        """
        if len(argv) == 1:
            return cases.Default.value

        self.file = None

        for arg in argv:
            match arg:
                case "--help":
                    return cases.Help.value
                case "-h":
                    return cases.Help.value
                case "-py":
                    self.file = argv[1]
                case _:
                    if not self.file:
                        self.file = arg
                        continue


    @property
    def open_file(self) -> str | Exception:
        """
        tries to open self.file from __init__
        """
        try:
            with open(self.file, 'r') as file:
                return file.read()
        except FileNotFoundError:
            self.panic(FileNotFoundError(f"could not open {self.file}"), suggestion="try using the absolute location of the file", with_traceback=True)
        except Exception as error:
            self.panic(error)