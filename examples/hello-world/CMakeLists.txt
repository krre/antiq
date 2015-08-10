cmake_minimum_required(VERSION 2.8)
project(hello-world)
set(CMAKE_CXX_FLAGS "-O0 -Wall -pedantic -std=c++11")
add_executable(hello-world main.cpp)
include_directories("${PROJECT_SOURCE_DIR}/../../src")
find_library(LIBS NAMES angie3d PATHS ${PROJECT_SOURCE_DIR}/../../lib)
target_link_libraries(hello-world ${LIBS})
install(FILES main.a3d DESTINATION ${PROJECT_BINARY_DIR})

