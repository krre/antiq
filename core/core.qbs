import qbs 1.0

DynamicLibrary {
    files: [
        "src/core.cpp",
        "src/core.h",
    ]
    name: "a3dcore"
    destinationDirectory: "lib"
    cpp.cxxLanguageVersion: "c++11"
    Depends { name: "cpp" }
}
