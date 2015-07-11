#pragma once

#include <string>
#include <vector>
#include "../global.h"

BEGIN_NAMESPACE_A3D

class Program
{
public:
    Program(std::string startFilePath);
    void compile();
    void run();

private:
    std::string startFilePath;
    std::vector<char> readSourceFromFile(std::string sourceFilePath);
};

END_NAMESPACE_A3D
