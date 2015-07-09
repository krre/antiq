#include "debug.h"
#include <iostream>

BEGIN_NAMESPACE_A3D

Debug::Debug()
{

}

template <typename T>
void Debug::print(T t)
{
    std::cout << t << std::endl;
}

template<typename T, typename... Args>
void Debug::print(T t, Args... args)
{
    std::cout << t << std::endl;
//    Debug::print(args...);
}

END_NAMESPACE_A3D

