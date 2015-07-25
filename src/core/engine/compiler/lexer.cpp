#include "lexer.h"
#include "token.h"
#include "../../debug.h"

Lexer::Lexer(std::string &source) : source(&source)
{
}

void Lexer::nextToken()
{
    value.clear();

    if (std::isspace(ch)) {
        scanSpace();
    }

    tokenPos = pos;

    if (std::isalpha(ch)) {
        scanIdent();
    }  else if (std::isdigit(ch)) {
        scanNumber();
    } else {
        switch (ch) {
        case -1: token = Token::EOT; break;
        case '"': scanString(); break;
        case '/':
            nextChar();
            if (ch == '/') {
                scanComment();
            } else {
                token = Token::DIV;
                nextChar();
            }
            break;
        case ':': token = Token::COLON; nextChar(); break;
        default: std::cerr << "(" << line << ", " << pos << ") Unexected symbol: " << ch << std::endl;
            exit(EXIT_SUCCESS);
        }
    }
}

void Lexer::nextChar()
{
    if (sourcePos == static_cast<int>(source->size() - 1)) {
        ch = -1;
        return;
    }
    sourcePos++;
    pos++;
    ch = source->at(sourcePos);
}

void Lexer::scanSpace()
{
    while (std::isspace(ch)) {
        if (ch == '\n') {
            line++;
            pos = 0;
        }
        nextChar();
    }
}

void Lexer::scanIdent()
{
    do {
        value += ch;
        nextChar();
    } while (std::isalnum(ch) || ch == '_');

    auto got = reserved.find(value);
    if (got == reserved.end()) {
        token = Token::NAME;
    } else {
        token = got->second;
    }
}

void Lexer::scanNumber()
{

}

void Lexer::scanString()
{
    nextChar();
    while (ch != '"' && ch != -1) {
        value += ch;
        nextChar();
    }
    if (ch == -1) {
        std::cerr << "Unexpected end of file" << std::endl;
        exit(EXIT_SUCCESS);
    } else {
        token = Token::STRING;
        nextChar();
    }
}

void Lexer::scanComment()
{

}


