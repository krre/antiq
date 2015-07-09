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

    m_currentFilePath = m_arguments.front();
    m_currentDirPath = m_currentFilePath.substr(0, m_currentFilePath.find_last_of("/\\"));
    m_currentFileName = m_currentFilePath.substr(m_currentFilePath.find_last_of("/\\") + 1);
}

END_NAMESPACE_A3D
