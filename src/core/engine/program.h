#pragma once

#include <string>
#include "../global.h"
#include "compiler/ast.h"

class Program
{
public:
    Program(std::string startFile);
    void compile();
    void run();

private:
    std::string readSource(std::string sourcePath);
    void evaluate(Ast *ast);
    std::string startFile;
    Ast *ast;
};

