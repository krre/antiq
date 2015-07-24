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
//    while (lexer->ch != EOF) {
        lexer->nextToken();
//        std::cout << lexer->ch << std::endl;
//    }


    return ast;
}
