#pragma once

#include <string>
#include <vector>
#include "../../global.h"
#include "ast.h"

BEGIN_NAMESPACE_A3D

class Parser
{
public:
    Parser();
    Ast *parseFile(std::string sourceFile);
    std::vector<char> readFile(std::string sourceFile);

};

END_NAMESPACE_A3D
