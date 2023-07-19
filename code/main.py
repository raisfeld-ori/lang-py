from rust_header import rust_header
from python_src import cli, config
import sys

if __name__ == '__main__':
    console = cli.Console(sys.argv)
    console.window.graceful_exit()