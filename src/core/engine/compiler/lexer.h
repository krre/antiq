#pragma once

#include <vector>
#include <string>
#include <unordered_map>
#include "../../global.h"
#include "token.h"

class Lexer
{
public:
    Lexer(std::string &source);
    void nextToken();
    Token token;
    std::string value;
    long long number;

private:
    void nextChar();
    void scanSpace();
    void scanIdent();
    void scanNumber();
    void scanString();
    void scanComment();

    std::string *source;
    char ch = ' '; // current char
    int pos = 0; // current pos at line
    int line = 1; // current line
    int sourcePos = -1; // current pos at source text
    int tokenPos = 0; // start token pos
    std::unordered_map<std::string, Token> keywords{
        { "break", Token::BREAK },
        { "case", Token::CASE },
        { "default", Token::DEFAULT },
        { "do", Token::DO },
        { "else", Token::ELSE },
        { "elsif", Token::ELSIF },
        { "end", Token::END },
        { "false", Token::FALSE },
        { "for", Token::FOR },
        { "func", Token::FUNC },
        { "import", Token::IMPORT },
        { "nil", Token::NIL },
        { "prop", Token::PROP },
        { "return", Token::RETURN },
        { "signal", Token::SIG },
        { "switch", Token::SWITCH },
        { "to", Token::TO },
        { "true", Token::TRUE },
        { "use", Token::USE },
        { "val", Token::VAL },
        { "var", Token::VAR },
        { "while", Token::WHILE },
    };
};

