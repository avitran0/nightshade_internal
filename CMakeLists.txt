cmake_minimum_required(VERSION 3.30)
project(nightshade LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE NIGHTSHADE_SRC "src/*.cpp")
add_library(nightshade SHARED ${NIGHTSHADE_SRC})
target_include_directories(nightshade PRIVATE "src")
target_compile_features(nightshade PRIVATE cxx_std_20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(OpenGL REQUIRED)

include(FetchContent)
FetchContent_Declare(
    sdl2
    GIT_REPOSITORY "https://github.com/libsdl-org/sdl"
    GIT_TAG "SDL2"
)
FetchContent_Declare(
    imgui
    GIT_REPOSITORY "https://github.com/ocornut/imgui"
    GIT_TAG "master"
)
FetchContent_MakeAvailable(imgui sdl2)

set(IMGUI_SRC
    ${imgui_SOURCE_DIR}/imgui.cpp
    ${imgui_SOURCE_DIR}/imgui_demo.cpp
    ${imgui_SOURCE_DIR}/imgui_draw.cpp
    ${imgui_SOURCE_DIR}/imgui_tables.cpp
    ${imgui_SOURCE_DIR}/imgui_widgets.cpp
    ${imgui_SOURCE_DIR}/backends/imgui_impl_sdl2.cpp
    ${imgui_SOURCE_DIR}/backends/imgui_impl_opengl3.cpp
)

add_library(imgui STATIC ${IMGUI_SRC})
target_include_directories(
    imgui PUBLIC
    ${imgui_SOURCE_DIR}
    ${imgui_SOURCE_DIR}/backends
)

target_link_libraries(
    imgui PUBLIC
    SDL2::SDL2
    OpenGL::GL
)

target_link_libraries(
    nightshade PRIVATE
    imgui
    SDL2::SDL2
)
