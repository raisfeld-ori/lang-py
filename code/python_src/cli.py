"""
cli:
the file that takes care of the cli interface.

it parses any args that is being passes into it,
and handles any errors that come with it.
"""
from sys import exit
from python_src.config import *


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


class handle_args():
    def __init__(self, argv: list[str]):
        """
        argv: sys.argv

        parses every arg in argv, and returns the appropriate
        result for every one of them
        """
        if len(argv) == 0:
            print(cases.Default.value)
            exit(0)

        self.file = None

        for arg in argv:
            match arg:
                case "--help":
                    print(cases.Help.value)
                    exit(0)
                case "-h":
                    print(cases.Help.value)
                    exit(0)
                case "-py":
                    self.file = argv[1]
                case _:
                    if not self.file:
                        self.file = arg
                        continue


    @property
    def file_data(self) -> str:
        try:
            with open(self.file, 'r') as file:
                return file.read()
        except Exception as error:
            print(error)