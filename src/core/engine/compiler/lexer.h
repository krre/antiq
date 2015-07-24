#pragma once

#include <vector>
#include <string>
#include "../../global.h"
#include "token.h"

class Lexer
{
public:
    Lexer(std::string *source);
    void nextToken();
    Token token;
    std::string value;

private:
    void nextChar();
    void skipSpace();

    std::string *source;
    char ch;
    unsigned int pos = -1;
    int line = 1;
};

