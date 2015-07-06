import qbs 1.0

DynamicLibrary {
    files: [
        "src/window.cpp",
        "src/window.h",
    ]
    name: "a3dui"
    destinationDirectory: "lib"
    Depends { name: "cpp" }
}
