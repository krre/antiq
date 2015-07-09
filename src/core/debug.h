#pragma once
#include "global.h"

BEGIN_NAMESPACE_A3D

class Debug
{
public:
    Debug();
    template<typename T, typename... Args>
    static void print(T t, Args... args);
    template <typename T>
    static void print(T t);
};

END_NAMESPACE_A3D
