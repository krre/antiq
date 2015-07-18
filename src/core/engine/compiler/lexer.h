#pragma once

#include <vector>
#include "../../global.h"
#include "token.h"

class Lexer
{
public:
    Lexer(std::vector<char> *source);
    void nextTok();
    Token token;

private:
    std::vector<char> *sourceData;
    char getChar();
    unsigned int pos = 0;
    int line = 1;
};

