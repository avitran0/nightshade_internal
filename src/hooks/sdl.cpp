#include "hooks/sdl.hpp"

#include "hooks/hooks.hpp"
#include "menu/menu.hpp"

SDLHook::SDLHook(const char *sdl_function_name, void *hook_function) {
    const auto sdl_handle = dlopen("libSDL2-2.0.so.0", RTLD_LAZY | RTLD_NOLOAD);
    const auto address = dlsym(sdl_handle, sdl_function_name);
    jump_target = reinterpret_cast<void **>(
        RelativeToAbsolute<uintptr_t>(reinterpret_cast<uintptr_t>(address) + 2));
    proxy = *jump_target;
    *jump_target = hook_function;
}

SDLHook::~SDLHook() {
    if (jump_target && proxy) {
        *jump_target = proxy;
    }
}

void SwapWindowHook(SDL_Window *window) {
    Menu::SwapBuffers(window);
    reinterpret_cast<SwapWindowFn>(Hooks::swap_window->proxy)(window);
}
