#pragma once

enum class Token {
    // identifiers and literals
    EOT, // end of text
    IDENT,
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

    // special symbols
    ASSIGN, // =
    COLON,
    COMMA,
    DIV,
    DOT,
    EQ, // ==
    GE, // >=
    GT, // >
    LB, // {
    LE, // <=
    LPAR, // (
    LT, // <
    MINUS,
    MOD,
    MULT,
    NE, // !=
    PLUS,
    QM, // ?
    RB, // }
    RPAR, // )
    SEMICOLON,
};
