#include <iostream>
#include <string>
#include "core/engine.h"
#include "core/application.h"

int main(int argc, char *argv[], char *envp[]) {
    a3d::Application app(argc, argv);
    a3d::Engine engine;
    std::string appDirPath = app.currentDirPath();
    engine.load(appDirPath + "/main.aml");
}
