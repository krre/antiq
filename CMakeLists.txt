cmake_minimum_required(VERSION 2.8)
project(angie3d)

set(VERSION_MAJOR "0")
set(VERSION_MINOR "1")
set(VERSION_PATCH "0")
set(VERSION "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")

set(CMAKE_CXX_FLAGS "-O0 -Wall -pedantic -std=c++11")

set(SOURCES "src/core/application.cpp"
            "src/core/application.h"
            "src/core/debug.cpp"
            "src/core/debug.h"
            "src/core/engine/compiler/ast.cpp"
            "src/core/engine/compiler/ast.h"
            "src/core/engine/compiler/parser.cpp"
            "src/core/engine/compiler/parser.h"
            "src/core/engine/compiler/token.h"
            "src/core/engine/engine.cpp"
            "src/core/engine/engine.h"
            "src/3dui/window.h"
            "src/3dui/window.cpp"
            "src/core/engine/compiler/lexer.cpp"
            "src/core/engine/compiler/lexer.h"
            "src/core/engine/program.cpp"
            "src/core/engine/program.h"
            "src/core/global.h")

find_package(OpenGL REQUIRED)
if (NOT OPENGL_FOUND)
    message(FATAL_ERROR "OpenGL not found")
endif()
set(LIBS ${OPENGL_LIBRARIES})

find_library(GLFW3_LIB NAMES glfw3 PATHS ${PROJECT_SOURCE_DIR}/lib)
list(APPEND LIBS ${GLFW3_LIB})

if (UNIX)
    find_package(X11 REQUIRED)
    list(APPEND LIBS ${X11_X11_LIB})
    if (NOT X11_Xrandr_FOUND)
        message(FATAL_ERROR "Randr library not found")
    endif()
    list(APPEND LIBS ${X11_Xrandr_LIB})

    if (NOT X11_Xinerama_FOUND)
        message(FATAL_ERROR "Xinerama library not found")
    endif()
    list(APPEND LIBS ${X11_Xinerama_LIB})

    if (NOT X11_Xcursor_FOUND)
        message(FATAL_ERROR "Xcursor library not found")
    endif()
    list(APPEND LIBS ${X11_Xcursor_LIB})

    if (NOT X11_Xinput_FOUND)
        message(FATAL_ERROR "Xinput library not found")
    endif()
    list(APPEND LIBS ${X11_Xinput_LIB})


endif()

add_library(angie3d SHARED ${SOURCES})

if (WIN32)
    set_target_properties(angie3d PROPERTIES PREFIX "")
endif()

target_link_libraries(angie3d ${LIBS})
install(TARGETS angie3d
   LIBRARY DESTINATION ${PROJECT_SOURCE_DIR}/lib
   RUNTIME DESTINATION ${PROJECT_SOURCE_DIR}/lib
)

