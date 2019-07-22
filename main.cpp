#include "include/glad/glad.h"
#define GLFW_DLL
#include "include/GLFW/glfw3.h"

#include <iostream>

int main(void) {
    const int nx = 2048;
    const int ny = 1024;

    if (!glfwInit()) {
        std::cout << "Could not init GLFW" << std::endl;
    }
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    GLFWwindow* window = glfwCreateWindow(nx, ny, "Ray GL", NULL, NULL);
    if (!window) {
        std::cout << "Could not init GLFW window" << std::endl;
        return -1;
    }
    glfwMakeContextCurrent(window);
    if(!gladLoadGLLoader((GLADloadproc) glfwGetProcAddress)) {
        std::cout << "Could not init OpenGL context" << std::endl;
        return -1;
    }

    glfwTerminate();
    return 0;
}
