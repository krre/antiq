#include "application.h"
#include <iostream>

BEGIN_NAMESPACE_A3D

Application::Application(int argc, char *argv[], char *envp[])
{
    for (int i = 0; argv[i] != nullptr; i++) {
        m_arguments.push_back(argv[i]);
    }

    if (envp != nullptr) {
        for (int i = 0; envp[i] != nullptr; i++) {
            m_environments.push_back(envp[i]);
        }
    }
}

END_NAMESPACE_A3D
