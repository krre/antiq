#pragma once

#include <string>
#include "../global.h"
#include "compiler/ast.h"

BEGIN_NAMESPACE_A3D

class Program
{
public:
    Program(std::string startFile);
    void run();

private:
    void evaluate(Ast &ast);
    std::string startFile;
};

END_NAMESPACE_A3D
