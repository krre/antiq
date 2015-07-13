import qbs 1.0
import qbs.FileInfo

Project {
    minimumQbsVersion: "1.4"

    DynamicLibrary {
        property var dependLibs: {
            var result = [];
            result.push("glfw3")
            if (qbs.targetOS.contains("windows"))
                result.push("opengl32")
            else if (qbs.targetOS.contains("linux"))
                result.push("GL",
                            "X11",
                            "Xinerama",
                            "Xcursor",
                            "Xrandr",
                            "Xi",
                            "Xxf86vm",
                            "pthread");
            return result
        }

        name: "angie3d"
        cpp.cxxLanguageVersion: "c++11"
        cpp.libraryPaths: "lib"
        cpp.dynamicLibraries: dependLibs
        qbs.installRoot: sourceDirectory

        files: [
            "src/core/application.cpp",
            "src/core/application.h",
            "src/core/debug.cpp",
            "src/core/debug.h",
            "src/core/engine/engine.cpp",
            "src/core/engine/engine.h",
            "src/3dui/window.h",
            "src/3dui/window.cpp",
            "src/core/engine/compiler/lexer.cpp",
            "src/core/engine/compiler/lexer.h",
            "src/core/engine/program.cpp",
            "src/core/engine/program.h",
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
