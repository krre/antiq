#pragma once

#include <string>
#include <vector>
#include "ast.h"

class Parser
{
public:
    Parser();
    Ast *parseFile(std::string sourceFile);
    std::vector<char> readFile(std::string sourceFile);

};
