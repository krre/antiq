#include <fstream>
#include "program.h"
#include "compiler/lexer.h"
#include "../../3dui/window.h"
#include "../debug.h"

BEGIN_NAMESPACE_A3D

Program::Program(std::string startFilePath): startFilePath(startFilePath)
{

}

void Program::compile()
{
    std::vector<char> &&sourceBuffer = readSourceFromFile(startFilePath);
    Lexer lexer(sourceBuffer);
    sourceBuffer.clear();
}

void Program::run()
{
//    Window window;
}

std::vector<char> Program::readSourceFromFile(std::string sourceFilePath)
{
    std::ifstream inFile{sourceFilePath, std::ios::in | std::ios::binary};

    if (!inFile.is_open()) {
        DBG << "Failed open file:" << sourceFilePath;
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

    inFile.close();

    return buffer;
}

END_NAMESPACE_A3D
