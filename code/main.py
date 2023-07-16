from rust_header import rust_header
import code.python_src.cli as cli
import sys

if __name__ == '__main__':
    console = cli.console(sys.argv)
    console.log(console.open_file)
    console.window.graceful_exit()