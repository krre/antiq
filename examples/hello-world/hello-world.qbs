import qbs 1.0

CppApplication {
    name: "hello-world"
    files: [
        "main.cpp",
    ]
    cpp.includePaths: "../../src"
    cpp.libraryPaths: "../../bin"
    cpp.dynamicLibraries: "angie3d"

    Group {
        name: "aml-install"
        qbs.install: true
        qbs.installDir: "../../../../bin"
        files: [
            "main.aml",
        ]
    }

    Group {
        qbs.install: true
        qbs.installDir: "../../../../bin"
        fileTagsFilter: "application"
    }
}
