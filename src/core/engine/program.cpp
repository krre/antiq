#include "program.h"
#include "compiler/parser.h"
#include "../../3dui/window.h"
#include "../debug.h"

Program::Program(std::string startFile): startFile(startFile)
{
}

void Program::run()
{
    Parser parser;
    Ast *ast = parser.parseFile(startFile);
    evaluate(*ast);
}

void Program::evaluate(Ast &ast)
{
    //    Window window;

}

