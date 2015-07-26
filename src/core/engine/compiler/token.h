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
    SIGNAL,
    SWITCH,
    TO,
    TRUE,
    USE,
    VAL,
    VAR,
    WHILE,

    // special symbols
    ASSIGN, // =
    COLON, // :
    COMMA, // ,
    DOT, // .
    DOT_DOT, // ..
    EQUALS, // ==
    GREATER_EQUALS, // >=
    GREATER_THEN, // >
    LEFT_BRACE, // {
    LEFT_BRACKET, // [
    LEFT_PAREN, // (
    LESS_EQUALS, // <=
    LESS_THEN, // <
    MINUS, // -
    NOT_EQUALS, // !=
    PLUS, // +
    QUESTION_MARK, // ?
    RIGHT_BRACE, // }
    RIGHT_BRACKET, // ]
    RIGHT_PAREN, // )
    SEMICOLON, // ;
    SLASH, // /
    STAR, // *
};
