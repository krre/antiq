import qbs 1.0

DynamicLibrary {
    files: [
        "src/window.cpp",
        "src/window.h",
    ]
    name: "a3dui"
    destinationDirectory: "lib"
    cpp.cxxLanguageVersion: "c++11"
    Depends { name: "cpp" }
}
