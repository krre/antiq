#pragma once

enum class Token {
    EOT, // end of text
    NAME,
    NONE,
    NUMBER,
    STRING,

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
