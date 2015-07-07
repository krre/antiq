import qbs 1.0

CppApplication {
    name: "hello-world"
    files: [
        "main.aml",
        "main.cpp",
    ]
    cpp.includePaths: "../../src"
    cpp.libraryPaths: "../../lib"
    cpp.dynamicLibraries: "angie3d"
}
