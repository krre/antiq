#include <iostream>
#include <string>
#include "core/engine.h"

int main(int argc, char **argv) {
    Engine engine;
    std::string path = (std::string)argv[0];
    engine.load(path + "/main.aml");
}
