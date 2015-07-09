#include <iostream>
#include <string>
#include "core/engine.h"
#include "core/application.h"
#include "core/debug.h"
#include "core/global.h"

template <typename T>
void func(T t)
{
    std::cout << t << " ";
}

template<typename T, typename... Args>
void func(T t, Args... args) // recursive variadic function
{
    std::cout << t << " ";
    func(args...);
    std::cout << std::endl;
}

int main(int argc, char *argv[], char *envp[]) {

    A3D_UNUSED(envp)

    a3d::Application app(argc, argv);
    a3d::Engine engine;
    std::string appDirPath = app.currentDirPath();
    engine.load(appDirPath + "/main.aml");
//    a3d::Debug::print(5, 0, "fsdfsdf", 1.6);

    func(5, 0, "fsdfsdf", 1.6);
}
