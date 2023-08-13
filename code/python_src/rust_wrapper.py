# wraps the rust code in order to make it easier to work with
from lang_py.lang_py import *


class BaseOutput:
    """
    a class for handling the output of the lang_py.parse.initial_parse function
    """
    def __init__(self, base_output: parse.BaseOutput):
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


class Method:
    """
    a wrapper for the Method class
    """
    def __init__(self, method: parse.Method):
        self.raw_method = method
        self.name = method.name()
        self.input = method.input()
        self.output = method.output()
        self.derivatives = method.derivatives()
        self.lines = sorted(method.lines(), key=lambda line: line.position())
        self.spaces = method.actual_line().actual_line().all_spaces()
        self.string_line = method.actual_line().actual_line().actual_line()


def handle_output(rust_output) -> BaseOutput:
    """
    since the rust code is just raw output, I made this to organize the output
    :param rust_output: the code from the rust lang_py
    :return: an output class from this file
    """
    match rust_output.output_type():
        case parse.AllOutputs.BaseOutput:
            return BaseOutput(rust_output)
