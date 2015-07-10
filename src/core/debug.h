#pragma once
#include "global.h"

BEGIN_NAMESPACE_A3D

// Singleton
class Debug
{
public:
    static Debug *instance();
    Debug *print();
private:
    Debug();
    static Debug *m_instance;
};

END_NAMESPACE_A3D
