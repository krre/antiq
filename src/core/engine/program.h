#pragma once

#include <string>
#include "compiler/ast.h"

class Program
{
public:
    Program(std::string startFile);
    void run();

private:
    void evaluate(Ast &ast);
    std::string startFile;
};

