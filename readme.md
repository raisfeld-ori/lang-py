# lang-py
lang py is a free python 
module for parsing
and compiling python code. 
the goal of the module is to 
assist in creating 
python compilers that
compile into other languages. 
also, unlike regular 
compilers, lang-py is an 
actual package that can be imported, 
so it's possible to directly
interfere with the compiler 
at any point, and do
things like additional optimization 
or fixing bugs.
also, the module
has both automatic,
and primitive methods
for everything.
so you can decide how 
deep you want to 
personally handle
the compilation, and
how much do you want to
leave to us.
also, most of the automatic
functions are async
with no GIL, so they are
much faster than
normal python functions.

# how do i use lang-py?
as said before, lang-py
allows the person using it
to choose how deep do you
want to manage the code,
and how much do you want
to leave to the compiler.
for convenience, you can
see both options here:
### leaving the code to the compiler:
if you want to have
no control over the compiler,
and you just want it to work,
you can use the following code
to start:
```python
from lang_py import lang_py
FILE = "example_file.py"
if __name__ == '__main__':
    with open(FILE, 'r') as file_data:
        # parses the code without the types
        example_file = lang_py.actions.async_parse_file(file_data.read(), FILE)
```