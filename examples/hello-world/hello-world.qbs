import qbs 1.0

CppApplication {
    name: "hello-world"
    files: [
        "main.cpp",
    ]
    cpp.includePaths: "../../src"
    cpp.libraryPaths: "../../lib"
    cpp.dynamicLibraries: "angie3d"

    Group {
        qbs.install: true
        files: [
            "main.aml",
        ]
    }

    Group {
        qbs.install: true
        fileTagsFilter: "application"
    }
}
