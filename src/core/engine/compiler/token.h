#pragma once

#include <string>

enum class Symbol {
    UNDEFINED,
    IDENT,
    NUMBER,
    WORD,
    END
};

struct Token
{
    Symbol sym;
    std::string value;
};
