# wraps the rust code in order to make it easier to work with
from compiler import compiler


def handle_output(rust_output):
    match rust_output.output_type():
        case compiler.classes.AllOutputs.BaseOutput:
            return BaseOutput(rust_output)


class BaseOutput:
    def __init__(self, base_output: compiler.classes.BaseOutput):
        self.variables: list[tuple] = base_output.variables()
        self.statements: list[tuple] = base_output.statements()
        self.executables: list[tuple] = base_output.executables()
        self.unknown: list[tuple] = base_output.unknown()
        self.all: list = sorted(self.variables + self.statements + self.executables + self.unknown,
                                key=lambda tp: tp[1])

    def __repr__(self) -> str:
        return f"{self.all}"
