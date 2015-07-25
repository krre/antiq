#include "core/engine/program.h"
#include <string>
#include <iostream>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        std::cout << "AngieLogic Interpreter" << std::endl;
        std::cout << "Usage: al [sourcefile]" << std::endl;

    } else {
        std::string filePath = argv[1];
        Program program(filePath);
        program.compile();
        program.run();
    }
}
