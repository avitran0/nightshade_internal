#include "menu/menu.hpp"

#include <SDL2/SDL_video.h>
#include <imgui.h>
#include <imgui_impl_opengl3.h>
#include <imgui_impl_sdl2.h>

#include <algorithm>
#include <mutex>

void Menu::Init() {
    ImGui::CreateContext();
    auto io = ImGui::GetIO();
    io.IniFilename = nullptr;
    io.LogFilename = nullptr;
}

void Menu::Shutdown() {
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplSDL2_Shutdown();
    ImGui::DestroyContext();
}

void Menu::SwapBuffers(SDL_Window *window) {
    static std::once_flag init;
    std::call_once(init, [window]() {
        ImGui_ImplSDL2_InitForOpenGL(window, SDL_GL_GetCurrentContext());
        ImGui_ImplOpenGL3_Init();
    });

    int width, height;
    SDL_GetWindowSize(window, &width, &height);

    auto io = ImGui::GetIO();
    io.DisplaySize = ImVec2(static_cast<float>(width), static_cast<float>(height));
    io.MousePos = ImVec2(
        std::clamp(io.MousePos.x, 0.0f, static_cast<float>(width)),
        std::clamp(io.MousePos.y, 0.0f, static_cast<float>(height)));

    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplSDL2_NewFrame();
    ImGui::NewFrame();

    // todo: input handling, etc
    ImGui::Begin("window title");
    ImGui::Text("nice text");
    ImGui::End();

    ImGui::Render();
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
}
