#include "lexer.h"
#include "token.h"
#include "../../debug.h"

Lexer::Lexer(std::vector<char> *source) : sourceData(source)
{
}

void Lexer::nextToken()
{
    token = {Symbol::UNDEFINED, ""};
    while (token.sym == Symbol::UNDEFINED) {
        char c = getChar();
        if (pos == sourceData->size() - 1) {
            token = {Symbol::END, ""};
        } else if (c == '\n') {
            line++;
        } else if (c == '{') {
            token = {Symbol::LB, ""};
        } else if (c == '}') {
            token = {Symbol::RB, ""};
        } else {
            DBG << line << "Unexected symbol:" << c;
            exit(EXIT_SUCCESS);
        }
        DBG << c;
    }
}

char Lexer::getChar()
{
    return sourceData->at(pos++);
}
