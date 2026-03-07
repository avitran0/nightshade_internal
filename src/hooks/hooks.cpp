#include "hooks/hooks.hpp"

#include "hooks/sdl.hpp"

void Hooks::Init() {
    swap_window = new SDLHook("SDL_GL_SwapWindow", reinterpret_cast<void *>(SwapWindowHook));
}

void Hooks::Uninit() {
    delete swap_window;
}
