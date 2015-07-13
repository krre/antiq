#include <fstream>
#include "parser.h"
#include "../../debug.h"

BEGIN_NAMESPACE_A3D

Parser::Parser()
{

}

Ast *Parser::parseFile(std::string sourceFile)
{
    Ast *ast = new Ast();
    std::vector<char> sourceBuffer = readFile(sourceFile);

//    sourceBuffer.clear();

    return ast;
}

std::vector<char> Parser::readFile(std::string sourceFile)
{
    std::ifstream inFile{sourceFile, std::ios::in | std::ios::binary};

    if (!inFile.is_open()) {
        DBG << "Failed open file:" << sourceFile;
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

