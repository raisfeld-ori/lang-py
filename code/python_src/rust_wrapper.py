# wraps the rust code in order to make it easier to work with
from compiler import compiler


class BaseOutput:
    """
    a class for handling the output of the compiler.parse.initial_parse function
    """
    def __init__(self, base_output: compiler.classes.BaseOutput):
        self.variables: list[tuple] = base_output.variables()
        self.statements: list[tuple] = base_output.statements()
        self.executables: list[tuple] = base_output.executables()
        self.unknown: list[tuple] = base_output.unknown()
        self.all: list = list(map(lambda arr: arr[0],
                              sorted(self.variables + self.statements + self.executables + self.unknown,
                              key=lambda tp: tp[1])))

    def __repr__(self) -> str:
        return f"{self.all}"


def handle_output(rust_output) -> BaseOutput:
    """
    since the rust code is just raw output, I made this to organize the output
    :param rust_output: the code from the rust compiler
    :return: an output class from this file
    """
    match rust_output.output_type():
        case compiler.classes.AllOutputs.BaseOutput:
            return BaseOutput(rust_output)
