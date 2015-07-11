#include "engine.h"
#include "program.h"

BEGIN_NAMESPACE_A3D

Engine::Engine()
{
}

void Engine::load(std::string startFilePath)
{
    Program program(startFilePath);
    program.compile();
    program.run();
}

END_NAMESPACE_A3D
