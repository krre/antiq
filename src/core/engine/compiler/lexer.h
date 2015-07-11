#pragma once

#include <vector>
#include "../../global.h"

BEGIN_NAMESPACE_A3D

class Lexer
{
public:
    Lexer(const std::vector<char> &sourceData);
private:
    void scan(const std::vector<char> &sourceData);
};

END_NAMESPACE_A3D
