#include "lexer.h"
#include "token.h"
#include "../../debug.h"

Lexer::Lexer(std::vector<char> *source) : sourceData(source)
{
}

void Lexer::nextToken()
{
    token = Token::UNDEFINED;
    while (token == Token::UNDEFINED) {
        nextChar();
        if (pos == sourceData->size() - 1) {
            token = Token::END;
        } else if (ch == '\n') {
            line++;
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
}

void Lexer::nextChar()
{
    ch = sourceData->at(pos++);
}
