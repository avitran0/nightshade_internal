cmake_minimum_required(VERSION 3.30)
project(nightshade LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE NIGHTSHADE_SRC "src/*.cpp")
add_library(nightshade SHARED ${NIGHTSHADE_SRC})
target_include_directories(nightshade PRIVATE "src")
target_compile_features(nightshade PRIVATE cxx_std_20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
