#include "hooks/hooks.hpp"
#include "interfaces/interfaces.hpp"
#include "menu/menu.hpp"
#include "util/log.hpp"

__attribute__((constructor)) void init() {
    Log::Info("loading nightshade");

    Menu::Init();
    Interfaces::Init();
    Hooks::Init();

    Log::Info("is in game: {}", Interfaces::engine->IsInGame());
    Log::Info("loaded nightshade");
}

__attribute__((destructor)) void exit() {
    Log::Info("unloading nightshade");
    Menu::Shutdown();
    Hooks::Uninit();
    Log::Info("unloaded nightshade");
}
