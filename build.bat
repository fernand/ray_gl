@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64

IF NOT EXIST build mkdir build
pushd build

del *.pdb > NUL 2> NUL

set compilerFlags=-Z7 -FC
cl %compilerFlags% ..\main.cpp ..\include\glad\glad.c /link ..\glfw3dll.lib
