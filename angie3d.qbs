import qbs 1.0
import qbs.FileInfo

Project {
    minimumQbsVersion: "1.4"

    DynamicLibrary {
        name: "angie3d"
        cpp.cxxLanguageVersion: "c++11"
        qbs.installRoot: sourceDirectory

        files: [
            "src/core/application.cpp",
            "src/core/application.h",
            "src/core/engine.cpp",
            "src/core/engine.h",
            "src/core/core.cpp",
            "src/core/core.h",
            "src/3dui/window.h",
            "src/3dui/window.cpp",
            "src/core/global.h",
        ]

        Depends { name: "cpp" }

        Group {
            qbs.install: true
            qbs.installDir: "lib"
            fileTagsFilter: product.type
        }
    }
}
