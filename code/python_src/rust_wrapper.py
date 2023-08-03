# wraps the rust code in order to make it easier to work with
from compiler import compiler


class Output:
    def __init__(self, rust_output):
        self.raw_output = rust_output

        match rust_output.output_type():
            case compiler.classes.AllOutputs.BaseOutput:
                ...