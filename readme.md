# the lang-py compiler
python is known for having a lot of great things, from modules
like numpy, to the simple syntax that makes it easier to
create large and complicated algorithms. one of it's biggest
disadvantages is the Cpython interpreter, since the interpreter
is very slow and stores everything on the heap, it's very bad
for performance. so instead, why not just code in python, do
testings in python, and once you're done, you can just compile
python code into a language like java, rust or go, which 
are known for their great compilers. this is what the lang-py
compiler is meant to offer.

## how to use the compiler
sadly, the compiler isn't complete yet, but, if you read
through the compiler (in the src directory) you will be able
to use the different components of the compiler in order to
get some insight and some outputs on your python code.

## how to assist with the project
the project doesn't have many dependencies, so if you
know rust or python, you can just go into the code and edit it.
however, it still has 2 important dependencies:
1. windows-curses - a python module for the cli
2. maturin/pyo3 - for compiling the rust code into python code.

so if you plan on editing the CLI, please read the curses
docs, and if you plan on helping with the rust code, please
read the maturin and pyo3 docs.