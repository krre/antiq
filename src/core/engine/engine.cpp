#include "engine.h"
#include "program.h"

BEGIN_NAMESPACE_A3D

Engine::Engine()
{
}

void Engine::load(std::string startFile)
{
    Program program(startFile);
    program.compile();
    program.run();
}

END_NAMESPACE_A3D
