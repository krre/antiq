#pragma once
#include "global.h"
#include <iostream>

BEGIN_NAMESPACE_A3D

#define DBG a3d::Debug()
#define debug(x) std::cout << x << std::endl;

class Debug
{
public:
    Debug();
    ~Debug() { std::cout << std::endl; }
    inline Debug &operator<<(bool t) { std::cout << (t ? "true" : "false") << " "; return *this; }
    inline Debug &operator<<(int t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(unsigned int t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(float t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(double t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(char t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(const char *t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(std::string t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(void *t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(std::nullptr_t) { std::cout << "(nullPtr)" << " "; return *this; }
};

END_NAMESPACE_A3D
