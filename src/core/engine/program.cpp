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
    std::string &&source = readSource(startFile);
    Lexer lexer(source);
    Parser parser(lexer);
    ast = parser.parse();
}

void Program::run()
{
    evaluate(ast);
}

void Program::evaluate(Ast *ast)
{
    // Window window;
    A3D_UNUSED(ast)

}

std::string Program::readSource(std::string sourcePath)
{
    std::ifstream in(sourcePath);
    if (in.is_open()) {
        std::string source((std::istreambuf_iterator<char>(in)), (std::istreambuf_iterator<char>()));
        if (source.empty()) {
            std::cerr << "File is empty: " << sourcePath << std::endl;
            exit(EXIT_FAILURE);
        } else {
            return source;
        }
    } else {
        std::cerr << "Failed open file: " << sourcePath << std::endl;
        exit(EXIT_FAILURE);
    }
}

