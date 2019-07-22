@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"

IF NOT EXIST build mkdir build
pushd build

del *.pdb > NUL 2> NUL

cl ..\main.cpp ..\include\glad\glad.c /link ..\glfw3dll.lib
