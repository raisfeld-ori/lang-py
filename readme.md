# the lang-py compiler
if you ever used python, you know how slow it is,
the reason for the slowness is that python is that python uses
an interpreter that compiles the code into c code, line by line.
while there are some python compilers (mainly pyinstaller),
they are simply not as good as an actual compiler, like rustc,
javac, gcc etc. which is why, instead of making a normal
compiler, this compiler turns your python code into other
programming languages.

## how do i use the compiler?
the compiler works the same as the pyinstaller compiler.
all you do is use ``pip install lang-py-compiler`` and
after pip finishes installing the code. just open up
your cmd/bash and write lang-py-compiler. if you want to
compile a file, just use ```lang-py-compiler yourfilename.py```