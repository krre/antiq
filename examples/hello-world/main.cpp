#include "core/engine.h"
#include "core/application.h"

int main(int argc, char *argv[]) {
    a3d::Application app(argc, argv);
    a3d::Engine engine;
    engine.load(app.currentDirPath() + "/main.aml");
}

