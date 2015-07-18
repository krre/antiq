#pragma once

#include <string>
#include <vector>
#include "../global.h"
#include "compiler/ast.h"

class Program
{
public:
    Program(std::string startFile);
    void compile();
    void run();

private:
    std::vector<char> *readFile(std::string sourceFile);
    void evaluate(Ast *ast);
    std::string startFile;
    Ast *ast;
};

