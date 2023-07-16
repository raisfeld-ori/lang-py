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
the compiler works similarly to pyinstaller, all you
need to do is ```pip install lang-py compiler``` and
then use ```lang-py-compiler 'your file name'``` and there
you go!, the file 