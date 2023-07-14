from enum import Enum

class language(Enum):
    rust = "rust"

class operating_system(Enum):
    windows = "Windows"

def from_file(file: str):
    raise NotImplementedError("THIS METHOD DOES NOT EXIST YET")