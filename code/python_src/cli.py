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


class ReprMethods(Enum):
    repr = auto()
    to_str = auto()
    name = auto()


class Cases(Enum):
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
    def __init__(self, std: curses.window, std_color=curses.COLOR_WHITE):
        self.std = std
        self.std.clear()
        self.std.refresh()
        self.std.keypad(True)
        self.current_color = std_color

    def cls(self):
        self.std.clear()
        self.std.refresh()

    @staticmethod
    def hide():
        curses.endwin()

    @staticmethod
    def show():
        curses.initscr()

    def write(self, obj: str) -> Exception:
        try:
            self.std.addstr(obj, self.current_color)
        except Exception as error:
            self.std.clear()
            self.std.refresh()
            self.write("< could not write to the std window >\n")
            return error

    def input(self) -> str:
        return self.std.getstr().decode()

    def edit_color(self, new_color):
        self.current_color = new_color

    def exit(self, state: int = 0):
        self.std.addstr("\npress enter to exit\n")
        self.std.getch()
        curses.endwin()
        sys.exit(state)


class Console:
    def __init__(self, argv: list[str]):
        """
        argv: sys.argv

        parses every arg in argv, and returns the appropriate
        result for every one of them
        """
        curses.initscr()
        curses.cbreak()
        curses.raw()
        curses.noecho()
        self.window = curses.wrapper(Window)
        self.window.std.keypad(True)
        self.height, self.width = self.window.std.getmaxyx()
        curses.resize_term(self.height, self.width)
        curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_YELLOW, curses.COLOR_BLACK)
        curses.init_pair(3, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(4, curses.COLOR_GREEN, curses.COLOR_BLACK)
        self.NORMAL_COLOR = curses.color_pair(1)
        self.WARNING_COLOR = curses.color_pair(2)
        self.ERROR_COLOR = curses.color_pair(3)
        self.SUCCESS_COLOR = curses.color_pair(4)
        self.warnings = 0

        self.print_log_errs = False
        self.print_too_large = False

        self._handle_args(argv)

    def warn(self, error: Exception | str, with_traceback: bool = False, suggestion: str = None):
        """
        like panic, but doesn't close the application
        :param error: the error you want to warn about
        :param with_traceback: whether you want to show traceback or not
        :param suggestion: an extra line of what could solve the issue
        :return: nothing
        """
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
        """
        logs an error in red, and then exits the application
        :param error: the error you want to panic about
        :param with_traceback: like the name says, determines if to show traceback or not
        :param suggestion: an extra line of what could solve the issue
        :return: nothing
        """
        try:
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
                self.log("suggestion -> ", suggestion)
            self.window.exit(1)
        except Exception as error:
            self.log(error)

    def log(self, *args, end: str = "\n", method: ReprMethods = ReprMethods.repr):
        """
        similar to print, but for curses instead
        :param args:
        the thing you want to print out
        :param end:
        how the print function will end
        :param method:
        curses only allows to write str objects,
        so you can select what method to turn the object into str
        (Repr is the normal method that python does)
        :return nothing
        """
        try:
            for arg in args:
                fail = None
                if type(arg) == str:
                    fail = self.window.write(arg)
                else:
                    match method:
                        case ReprMethods.repr:
                            fail = self.window.write(f"{arg}")
                        case ReprMethods.to_str:
                            fail = self.window.write(str(arg))
                        case ReprMethods.name:
                            fail = self.window.write(arg.__name__)
                if fail:
                    self.window.write("")
                    if self.print_too_large:
                        print(arg)
                    if self.print_log_errs:
                        print(fail)

            self.window.write(end)
        except Exception as error:
            self.panic(error, with_traceback=False, suggestion="try using another repr method")

    def select(self, question: str, options: list[str], return_int: bool = True) -> str | int:
        """
        allows to select from multiple options
        :param question:
        :param options:
        :param return_int:
        :return:
        """
        self.window.cls()
        if len(options) < 2:
            raise ValueError("console.select must include more then 2 options")

        selected = 0
        while True:
            self.log(question, "\n")
            for i, option in enumerate(options):
                if i == selected:
                    self.window.edit_color(self.WARNING_COLOR)
                    self.log(">", end="")
                self.log(option)
                self.window.edit_color(self.NORMAL_COLOR)
            key = self.window.std.getch()
            if key == curses.KEY_UP:
                selected -= 1 if selected > 0 else 0
            elif key == curses.KEY_DOWN:
                selected += 1 if selected + 1 != len(options) else 0
            elif key == curses.KEY_ENTER or key == 13:
                self.window.cls()
                return selected if return_int else  options[selected]
            self.window.cls()

    def debug(
            self, print_log_errs: bool = False,
            print_too_large: bool = False):
        """
        :param print_log_errs: if an error happens while logging, print it
        :param print_too_large: if something is too large for the console, print it
        :return: nothing
        """
        self.print_log_errs = print_log_errs
        self.print_too_large = print_too_large

    def graceful_exit(self, state: int = 0):
        self.window.edit_color(self.SUCCESS_COLOR)
        self.log("warnings: ", self.warnings)

        self.window.exit(state)

    def _handle_args(self, argv: list[str]):
        """
        handles the arguments for what should print out
        :parm argv: takes in sys.argv, so that custom handles are possible before this
        """
        if len(argv) == 1:
            self.log(Cases.Default.value)
            self.graceful_exit()

        self.file = None

        for arg in argv:
            match arg:
                case "--help":
                    self.log(Cases.Help.value)
                    self.graceful_exit(0)
                case "-h":
                    self.log(Cases.Help.value)
                    self.graceful_exit(0)
                case "-py":
                    self.file = argv[1]
                case _:
                    if not self.file:
                        self.file = arg
                        continue
        if not self.file:
            self.panic("no file given", suggestion="try using -py if you're using the python interpreter")

    def open_file(self) -> str | Exception:
        """
        tries to open the file from __init__
        """
        try:
            with open(self.file, 'r') as file:
                return file.read()
        except FileNotFoundError:
            self.panic(FileNotFoundError(f"could not open {self.file}"),
                       suggestion="try using the absolute location of the file",
                       with_traceback=True)
        except Exception as error:
            self.panic(error, with_traceback=True)
