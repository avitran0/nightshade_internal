#pragma once

#include <SDL_events.h>
#include <SDL_video.h>

namespace Menu {
    inline bool open = false;

    void Init();
    void Shutdown();
    void SwapBuffers(SDL_Window *window);
    void ProcessEvent(SDL_Event *event);
};  // namespace Menu
