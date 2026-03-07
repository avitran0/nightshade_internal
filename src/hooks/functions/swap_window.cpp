#include "hooks/functions.hpp"
#include "hooks/hooks.hpp"
#include "menu/menu.hpp"

void SwapWindowHook(SDL_Window *window) {
    Menu::SwapBuffers(window);
    reinterpret_cast<SwapWindowFn>(Hooks::swap_window->proxy)(window);
}
