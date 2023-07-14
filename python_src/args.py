from enum import Enum
from sys import exit
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
        --help -> writes how to use the compiler, and the current kwargs
    
    WARNING:
        the compiler is in beta, and is bound to have a lot of errors and bugs
    
    """


class handle_args():
    def __init__(self, argv):
        if (argv[0] == __file__ and len(argv) == 1) or len(argv) == 0:
            print(cases.Default.value)
            exit(0)
        elif argv[0] == __file__:
            self.file = argv[1]
