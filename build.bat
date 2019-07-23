@echo off

IF NOT EXIST build mkdir build
pushd build

del *.pdb > NUL 2> NUL

set compilerFlags=-Z7 -FC
cl %compilerFlags% ..\main.cpp ..\include\glad\glad.c /link ..\glfw3dll.lib

popd
