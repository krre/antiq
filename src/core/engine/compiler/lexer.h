#pragma once

#include <vector>
#include <string>
#include "../../global.h"
#include "token.h"

class Lexer
{
public:
    Lexer(std::vector<char> *source);
    void nextToken();
    Token token;
    std::string value;

private:
    std::vector<char> *sourceData;
    char getChar();
    unsigned int pos = 0;
    int line = 1;
};

