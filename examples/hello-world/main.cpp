#include <iostream>
#include <string>
#include "core/engine.h"
#include "core/application.h"
#include "core/debug.h"
#include "core/global.h"

int main(int argc, char *argv[]) {
    a3d::Application app(argc, argv);
    a3d::Engine engine;
    std::string appDirPath = app.currentDirPath();
    engine.load(appDirPath + "/main.aml");
}
