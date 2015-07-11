#include "engine.h"
#include "debug.h"
#include "../3dui/window.h"

BEGIN_NAMESPACE_A3D

Engine::Engine()
{
}

void Engine::load(std::string filePath)
{
    DBG << filePath;
    Window window;
}

END_NAMESPACE_A3D
