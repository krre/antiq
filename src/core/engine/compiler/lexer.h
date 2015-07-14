#pragma once

#include <vector>

class Lexer
{
public:
    Lexer(const std::vector<char> &sourceData);
private:
    void scan(const std::vector<char> &sourceData);
};

