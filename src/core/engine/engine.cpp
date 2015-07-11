#include <fstream>
#include <vector>
#include "engine.h"
#include "../debug.h"
#include "../../3dui/window.h"

BEGIN_NAMESPACE_A3D

Engine::Engine()
{
}

void Engine::load(std::string filePath)
{
    std::ifstream inFile{filePath, std::ios::in | std::ios::binary};

    if (!inFile.is_open()) {
        DBG << "Failure open file:" << filePath;
        exit(EXIT_FAILURE);
    }

    std::vector<char> buffer;
    std::ifstream::pos_type size = 0;

    if (inFile.seekg(0, std::ios::end)) {
        size = inFile.tellg();
    }

    if (size && inFile.seekg(0, std::ios::beg)) {
        buffer.resize(size);
        inFile.read(&buffer[0], size);
    }

    DBG << buffer.size();

    inFile.close();

    Window window;
}

END_NAMESPACE_A3D
