cmake_minimum_required(VERSION 2.8)
set(LIB angie3d)
project(${LIB})

set(VERSION_MAJOR "0")
set(VERSION_MINOR "1")
set(VERSION_PATCH "0")
set(VERSION "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")

set(CMAKE_CXX_FLAGS "-O0 -Wall -pedantic -std=c++11")

file(GLOB_RECURSE SOURCES src/3dui/*.cpp
                          src/3dui/*.h
                          src/core/*.cpp
                          src/core/*.h
                          src/3rdparty/*.h
                          src/3rdparty/*.cpp
)

find_library(LIBS NAMES glfw3 PATHS ${PROJECT_SOURCE_DIR}/lib)

add_library(${LIB} SHARED ${SOURCES})
target_link_libraries(${LIB} ${LIBS})
install(TARGETS ${LIB} DESTINATION ${PROJECT_SOURCE_DIR}/lib)
