"""
config:
the config file stores all options and settings, in order to create, and edit
the current compiler settings all from the same file.
"""

from enum import Enum

class language(Enum):
    rust = "rust"

class operating_system(Enum):
    windows = "Windows"
