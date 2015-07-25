#include "lexer.h"
#include "token.h"
#include "../../debug.h"

Lexer::Lexer(std::string &source) : source(&source)
{
}

void Lexer::nextToken()
{
    value.clear();
    skipSpace();

    if (pos == source->size() - 1) {
        token = Token::END;
    } else if (ch == '{') {
        token = Token::LB;
    } else if (ch == '}') {
        token = Token::RB;
    } else {
        DBG << line << "Unexected symbol:" << ch;
        exit(EXIT_SUCCESS);
    }
    DBG << ch;
}

void Lexer::nextChar()
{
    ch = source->at(++pos);
}

void Lexer::skipSpace()
{
    while(1) {
        nextChar();
        if (ch == ' ' || ch == '\t' || ch == '\r') continue;
        else if (ch == '\n') line++;
        else break;
    }
}
