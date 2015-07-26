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
        case '(': token = Token::LEFT_PAREN; nextChar(); break;
        case ')': token = Token::RIGHT_PAREN; nextChar(); break;
        case '=':
            nextChar();
            if (ch == '=') {
                token = Token::EQUALS;
                nextChar();
            } else {
                token = Token::ASSIGN;
            }
            break;
        case '/':
            nextChar();
            if (ch == '/') {
                scanComment();
            } else {
                token = Token::SLASH;
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

    auto got = keywords.find(value);
    if (got == keywords.end()) {
        token = Token::IDENT;
    } else {
        token = got->second;
    }
}

void Lexer::scanNumber()
{
    while (std::isdigit(ch)) {
        value += ch;
        nextChar();
    }
    token = Token::NUMBER;
    nextChar();
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


