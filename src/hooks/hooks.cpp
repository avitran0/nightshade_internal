#include "hooks/hooks.hpp"

#include "hooks/functions.hpp"
#include "hooks/sdl.hpp"
#include "hooks/vmt.hpp"
#include "interfaces/interfaces.hpp"

void Hooks::Init() {
    swap_window = new SDLHook("SDL_GL_SwapWindow", reinterpret_cast<void *>(SwapWindowHook));
poll_event = new SDLHook("SDL_PollEvent", reinterpret_cast<void *>(PollEvent));

    draw_model_execute =
        new VMTHook(21, Interfaces::model_render, reinterpret_cast<void *>(DrawModelExecute));
}

void Hooks::Uninit() {
    delete swap_window;
    delete draw_model_execute;
}
