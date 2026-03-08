#pragma once

#include "hooks/sdl.hpp"
#include "hooks/vmt.hpp"

namespace Hooks {
    void Init();
    void Uninit();

    inline SDLHook *swap_window;
    inline SDLHook *poll_event;

    inline VMTHook *draw_model_execute;
}
