#pragma once

#include <SDL2/SDL_video.h>

namespace Menu {
    void Init();
    void Shutdown();
    void SwapBuffers(SDL_Window *window);
};
