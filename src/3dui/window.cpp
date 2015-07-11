#include "window.h"
#include "../core/debug.h"

BEGIN_NAMESPACE_A3D

Window::Window()
{
    DBG << "window";

    if (!glfwInit()) {
        exit(EXIT_FAILURE);
    }

    glWnd = glfwCreateWindow(width, height, title, NULL, NULL);

    if (!glWnd) {
        glfwTerminate();
        exit(EXIT_FAILURE);
    }

    glfwMakeContextCurrent(glWnd);
    glClearColor(1.0, 1.0, 1.0, 1.0);

    while (!glfwWindowShouldClose(glWnd)) {
        int width, height;
        glfwGetFramebufferSize(glWnd, &width, &height);
        glViewport(0, 0, width, height);
        glClear(GL_COLOR_BUFFER_BIT);
        glfwSwapBuffers(glWnd);
        glfwWaitEvents();
    }
}

Window::~Window()
{
    glfwDestroyWindow(glWnd);
    glfwTerminate();
}

END_NAMESPACE_A3D

