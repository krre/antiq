#pragma once

#include "../../global.h"
#include "ast.h"
#include "lexer.h"

class Parser
{
public:
    Parser(Lexer &lex);
    ~Parser();
    Ast *parse();

private:
    Lexer *lexer;
    Ast *ast;
};
