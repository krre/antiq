#pragma once

enum class Token {
    // identifiers and literals
    EOT, // end of text
    IDENT,
    NONE,
    NUMBER,
    STRING,

    // keywords
    BREAK,
    CASE,
    DEFAULT,
    DO,
    ELSE,
    ELSIF,
    END,
    FALSE,
    FOR,
    FUNC,
    IF,
    IMPORT,
    NIL,
    PROP,
    RETURN,
    SIG,
    SWITCH,
    TO,
    TRUE,
    USE,
    VAL,
    VAR,
    WHILE,

    // arithmetic
    DIV,
    EQ, // ==
    GE, // >=
    GT, // >
    LE, // <=
    LT, // <
    MINUS,
    MOD,
    MULT,
    NE, // !=
    PLUS,
    ASSIGN, // =

    // service
    COLON,
    COMMA,
    DOT,
    LB, // {
    LPAR, // (
    QM, // ?
    RB, // }
    RPAR, // )
    SEMICOLON,
};
