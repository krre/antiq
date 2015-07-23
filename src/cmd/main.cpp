#include "core/engine/program.h"
#include <string>
#include <iostream>

int main(int argc, char *argv[]) {
    if (argc > 1) {
        std::string filePath = argv[1];
        Program program(filePath);
        program.compile();
        program.run();
    } else {
        std::cout << "AngieLogic Interpreter" << std::endl;
    }
}
