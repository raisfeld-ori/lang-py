from rust_header import rust_header
from python_src import cli
import sys

if __name__ == '__main__':
    console = cli.Console(sys.argv)
    try:
        data = console.open_file
        result = rust_header.parse.initial_parse(data)
        console.log(result.value())
    except Exception as error:
        console.panic(error)
    finally:
        console.graceful_exit(0)