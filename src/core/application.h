#pragma once
#include "global.h"
#include <string>
#include <list>

BEGIN_NAMESPACE_A3D

class Application
{
public:
    Application(int argc, char *argv[], char *envp[] = nullptr);
    std::list<std::string> arguments() { return m_arguments; }
    std::list<std::string> environments() { return m_environments; }
    std::string currentFilePath() { return m_currentFilePath; }
    std::string currentDirPath() { return m_currentDirPath; }
    std::string currentFileName() { return m_currentFileName; }

private:
    std::list<std::string> m_arguments;
    std::list<std::string> m_environments;
    std::string m_currentFilePath;
    std::string m_currentDirPath;
    std::string m_currentFileName;
};

END_NAMESPACE_A3D
