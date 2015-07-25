#pragma once

enum class Token {
    NONE,
    IMPORT,
    USE,
    NAME,
    NUMBER,
    STRIGN,
    SIGNAL,
    PROP,
    VAR,
    VAL,
    FUNC,
    END,
    RETURN,
    EOT, // end of text

    WHILE,
    DO,
    IF,
    ELSE,
    ELSIF,

    MULT,
    DIV,
    MOD,
    PLUS,
    MINUS,
    EQ, // =
    NE, // !=
    LT, // <
    LE, // <=
    GT, // >
    GE, // >=

    DOT,
    COMMA,
    COLON,
    SEMICOLON,
    ASSIGN,
    QM, // ?
    LPAR, // (
    RPAR, // )
    LB, // {
    RB // }
};
