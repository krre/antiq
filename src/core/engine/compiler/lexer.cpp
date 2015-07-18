#include "lexer.h"
#include "token.h"
#include "../../debug.h"

Lexer::Lexer(std::vector<char> *source) : sourceData(source)
{
//    token.sym = Symbol.UNDEFINED;
}

void Lexer::nextTok()
{
    while (1) {
        char c = getChar();
        DBG << c;
        if (c == 0) {
            break;
        }
    }
}

char Lexer::getChar()
{
    return sourceData->at(pos++);
}
