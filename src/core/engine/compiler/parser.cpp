#include "parser.h"
#include "../../debug.h"

Parser::Parser(Lexer *lex): lexer(lex)
{
    ast = new Ast();
}

Parser::~Parser()
{
    delete ast;
}

Ast *Parser::parse()
{
    lexer->nextTok();

    return ast;
}
