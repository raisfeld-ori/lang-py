# lang-py
lang-py is a parser for python code, in order to create a compiler. it uses
rust for all of it's code, so it's very fast and avoids python's GIL by using tokio threads.
if you plan on compiling python code into any other format, you can use lang-py to do
things from basic parsing to optimizing the output.

## how to use lang-py
(WARNING: this tutorial is incomplete, since this is version 0.1.0)\
first, use ```from lang_py import lang_py```. then, start by parsing
the basic structure of the code using ```lang_py.parse.initial_parse(your_python_code)```.
this will parse the code and give you the class ```lang_py.classes.BaseOutput```.
which has basic parsing on things like variables, functions, statements etc. from this point,
you can use get_methods