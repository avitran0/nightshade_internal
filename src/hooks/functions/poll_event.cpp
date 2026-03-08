#include <SDL_events.h>

#include "hooks/functions.hpp"
#include "hooks/hooks.hpp"
#include "menu/menu.hpp"

int PollEvent(SDL_Event *event) {
    if (Menu::open) {
        Menu::ProcessEvent(event);
        event->type = 0;
    }
    return reinterpret_cast<PollEventFn>(Hooks::poll_event->proxy)(event);
}
