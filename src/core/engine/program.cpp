#include <fstream>
#include "program.h"
#include "compiler/parser.h"
#include "compiler/lexer.h"
#include "../../3dui/window.h"
#include "../debug.h"

Program::Program(std::string startFile): startFile(startFile)
{
}

void Program::compile()
{
    std::vector<char> *sourceBuffer = readFile(startFile);

    Lexer lexer(sourceBuffer);
    Parser parser(&lexer);
    ast = parser.parse();

    sourceBuffer->clear();
    delete sourceBuffer;
}

void Program::run()
{
    evaluate(ast);
}

void Program::evaluate(Ast *ast)
{
    // Window window;

}

std::vector<char> *Program::readFile(std::string sourceFile)
{
    std::ifstream inFile{sourceFile, std::ios::in | std::ios::binary};

    if (!inFile.is_open()) {
        DBG << "Failed open file:" << sourceFile;
        exit(EXIT_FAILURE);
    }

    auto buffer = new std::vector<char>;
    std::ifstream::pos_type size = 0;

    if (inFile.seekg(0, std::ios::end)) {
        size = inFile.tellg();
    }

    if (size && inFile.seekg(0, std::ios::beg)) {
        buffer->resize(size);
        inFile.read(&(*buffer)[0], size);
    }

    inFile.close();

    return buffer;
}

