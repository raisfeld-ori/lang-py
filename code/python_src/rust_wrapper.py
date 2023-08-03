from compiler import compiler
from python_src.cli import Console
class output:
    def __init__(self, rust_output):
        self.raw_output = rust_output

        match rust_output.output_type():
            case compiler.classes.AllOutputs.BaseOutput:
                console.log("is basic")