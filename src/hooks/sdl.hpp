#pragma once

#include <dlfcn.h>

#include <cstdint>

#include "imgui_impl_sdl2.h"

template <typename T>
T RelativeToAbsolute(uintptr_t address) {
    return reinterpret_cast<T>(address + 4 + *reinterpret_cast<int32_t *>(address));
}

struct SDLHook {
    void **jump_target;
    void *proxy;

    SDLHook() = delete;
    SDLHook(const char *sdl_function_name, void *hook_function);
    ~SDLHook();
};

using SwapWindowFn = void (*)(SDL_Window *);
void SwapWindowHook(SDL_Window *window);
