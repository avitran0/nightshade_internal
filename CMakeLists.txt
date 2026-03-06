cmake_minimum_required(VERSION 3.30)
project(nightshade LANGUAGES CXX)

file(GLOB NIGHTSHADE_SRC "src/*.cpp")
add_library(nightshade SHARED ${NIGHTSHADE_SRC})
target_include_directories(nightshade PRIVATE "src")
target_compile_features(nightshade PRIVATE cxx_std_20)
