#include "debug.h"
#include <iostream>

BEGIN_NAMESPACE_A3D

Debug *Debug::m_instance = nullptr;

Debug::Debug()
{
    std::cout << "debug" << std::endl;
}

Debug *Debug::instance()
{
    if (m_instance == nullptr) {
        m_instance = new Debug();
    }

    return m_instance;
}

Debug *Debug::print()
{
    std::cout << "print" << std::endl;
    return this;
}

END_NAMESPACE_A3D

