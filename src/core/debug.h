#pragma once
#include "global.h"
#include <iostream>

BEGIN_NAMESPACE_A3D

#define DBG a3d::Debug::instance()

// Singleton
class Debug
{
public:
    static Debug &instance();
    inline Debug &operator<<(bool t) { std::cout << (t ? "true" : "false") << " "; return *this; }
    inline Debug &operator<<(int t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(float t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(double t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(const char *t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(void *t) { std::cout << t << " "; return *this; }
    inline Debug &operator<<(std::nullptr_t) { std::cout << "(nullPtr)" << " "; return *this; }
private:
    Debug();
    static Debug *m_instance;
};

END_NAMESPACE_A3D
