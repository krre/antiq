cmake_minimum_required(VERSION 2.8)
project(al)
add_executable(al main.cpp)
include_directories("../")
add_library(angie3d SHARED IMPORTED)
target_link_libraries(al angie3d)

