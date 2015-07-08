#include "engine.h"
#include <iostream>

BEGIN_NAMESPACE_A3D

Engine::Engine()
{
}

void Engine::load(std::string filePath)
{
    std::cout << filePath << std::endl;
}

END_NAMESPACE_A3D
