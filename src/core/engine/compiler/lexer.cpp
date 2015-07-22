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
        char c = nextChar();
        if (pos == sourceData->size() - 1) {
            token = Token::END;
        } else if (c == '\n') {
            line++;
        } else if (c == '{') {
            token = Token::LB;
        } else if (c == '}') {
            token = Token::RB;
        } else {
            DBG << line << "Unexected symbol:" << c;
            exit(EXIT_SUCCESS);
        }
        DBG << c;
    }
}

char Lexer::nextChar()
{
    return sourceData->at(pos++);
}
