#pragma once

#include "hooks/sdl.hpp"

namespace Hooks {
    void Init();
    void Uninit();

    inline SDLHook *swap_window;
}
