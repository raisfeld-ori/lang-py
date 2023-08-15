# wraps the rust code in order to make it easier to work with
from lang_py.lang_py import *


class BaseOutput:
    """
    a class for handling the output of the lang_py.parse.initial_parse function
    """
    def __init__(self, base_output: actions.BaseOutput):
        self.self = base_output
        self.variables: list[parse.BaseVar] = base_output.variables()
        self.statements: list[parse.BaseStatement] = base_output.statements()
        self.executables: list[parse.BaseExecutable] = base_output.executables()
        self.unknown: list[parse.ShallowParsedLine] = base_output.unknown()
        self.all: list = list((
            sorted(self.variables + self.statements + self.executables + self.unknown,
                              key=lambda line: line.actual_line().position())))
        self.shallow_code: list[parse.ShallowParsedLine] = base_output.shallow_code()

    def __repr__(self) -> str:
        return f"{self.all}"


def handle_output(rust_output) -> BaseOutput:
    """
    since the rust code is just raw output, I made this to organize the output
    :param rust_output: the code from the rust lang_py
    :return: an output class from this file
    """
    match rust_output.output_type():
        case actions.AllOutputs.BaseOutput:
            return BaseOutput(rust_output)

