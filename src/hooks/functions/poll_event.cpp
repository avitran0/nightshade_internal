#include <SDL_events.h>

#include "hooks/functions.hpp"
#include "hooks/hooks.hpp"
#include "menu/menu.hpp"

void PollEvent(SDL_Event *event) {
    if (Menu::open) {
        Menu::ProcessEvent(event);
        event->type = 0;
    }
    reinterpret_cast<PollEventFn>(Hooks::poll_event->proxy)(event);
}
