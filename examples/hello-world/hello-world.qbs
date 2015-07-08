import qbs 1.0

CppApplication {
    name: "hello-world"
    files: "main.cpp"
    cpp.includePaths: "../../src"
    cpp.libraryPaths: "../../lib"
    cpp.dynamicLibraries: "angie3d"
    qbs.installRoot: sourceDirectory

    Group {
        name: "Runtime resources"
        qbs.install: true
        qbs.installDir: "build"
        files: "*.aml"
    }

    Group {
        name: "The App itself"
        qbs.install: true
        qbs.installDir: "build"
        fileTagsFilter: "application"
    }
}
