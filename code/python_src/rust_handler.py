from compiler import compiler

class output:
    def __init__(self, rust_output):
        self.raw_output = rust_output

        match rust_output.output_type():
            case compiler.classes.AllOutputs.BaseOutput:
                ...