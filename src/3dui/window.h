#pragma once

#include "../core/global.h"
#include "../3rdparty/GLFW/glfw3.h"

BEGIN_NAMESPACE_A3D

class Window
{
public:
    Window();
    ~Window();

private:
    const char *title = "Angie3D";
    int width = 640;
    int height = 480;
    GLFWwindow *glWnd;
};

END_NAMESPACE_A3D
