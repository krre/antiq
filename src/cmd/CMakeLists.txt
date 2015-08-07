cmake_minimum_required(VERSION 2.8)
set(BIN al)
project(${BIN})
add_executable(${BIN} main.cpp)
include_directories("../")
find_library(LIBS NAMES angie3d PATHS ${PROJECT_SOURCE_DIR}/../../lib)
target_link_libraries(${BIN} ${LIBS})

