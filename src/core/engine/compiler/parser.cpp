#include "parser.h"
#include "../../debug.h"

Parser::Parser(Lexer &lex): lexer(&lex)
{
    ast = new Ast();
}

Parser::~Parser()
{
    delete ast;
}

Ast *Parser::parse()
{
    lexer->nextToken();
    while (lexer->token != Token::EOT) {
        debug("Token: " << lexer->value);

        switch (lexer->token) {
        case Token::IDENT:
            break;
        default:
            break;
        }

        lexer->nextToken();
    }

    return ast;
}
