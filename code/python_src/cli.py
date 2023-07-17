"""
cli:
the file that takes care of the cli interface.

it parses any args that is being passes into it,
and handles any errors that come with it.
"""
from code.python_src.config import *
import curses
import sys
from types import TracebackType
from traceback import extract_tb
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
class window:
    def __init__(self, std: curses.window, std_color: curses.A_COLOR):
        self.std = std
        self.std.clear()
        self.std.refresh()
        self.current_color = std_color

    def write(self, obj):
        self.std.addstr(obj, self.current_color)

    def edit_color(self, new_color):
        self.current_color = new_color

    def graceful_exit(self):
        self.std.addstr("\npress enter to exit\n")
        self.std.getch()
        sys.exit(0)


class console:
    def __init__(self, argv: list[str]):
        """
        argv: sys.argv

        parses every arg in argv, and returns the appropriate
        result for every one of them
        """
        curses.initscr()
        self.output = self._handle_args(argv)
        self.window = curses.wrapper(window)

        curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_YELLOW, curses.COLOR_BLACK)
        curses.init_pair(3, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(4, curses.COLOR_GREEN, curses.COLOR_BLACK)
        self.NORMAL_COLOR = curses.color_pair(1)
        self.WARNING_COLOR = curses.color_pair(2)
        self.ERROR_COLOR = curses.color_pair(3)
        self.SUCCESS_COLOR = curses.color_pair(4)


    def warn(self, error: Exception, description: str):
        self.window.edit_color(self.WARNING_COLOR)



    def log(self, *args, end: str = "\n", ignore_no_repr: bool = True) -> None:
        for arg in args:

            if type(arg) == str:
                self.window.write(arg)
            elif hasattr(arg, "__repr__"):
                self.window.write(arg.__repr__())
            elif ignore_no_repr:
                self.window.write(arg.__name__())

        self.window.write(end)



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
        except Exception as error:
            return error

def create_exceptions(name: str):
    class Error(Exception):
        def __init__(self, description):
            self.description = description
            self.name = name
            self.DEFAULT_TRACEBACK = TracebackType(

            )

        def __repr__(self) -> str:
            return f"{self.name}: {self.description}"

        def with_traceback(self, tb: TracebackType | None = None) -> str:
            print(extract_tb(tb))
            return "test"

    return Error