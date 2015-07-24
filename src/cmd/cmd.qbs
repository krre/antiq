import qbs 1.0

CppApplication {
    name: "al"
    files: "main.cpp"
    cpp.includePaths: "../"
    cpp.libraryPaths: "../../lib"
    cpp.dynamicLibraries: [
        "angie3d"
    ]
    cpp.cxxLanguageVersion: "c++11"
    qbs.installRoot: sourceDirectory

    Group {
        name: "The App itself"
        qbs.install: true
        qbs.installDir: "build"
        fileTagsFilter: "application"
    }
}
