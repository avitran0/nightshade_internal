#include "hooks/hooks.hpp"
#include "interfaces/interfaces.hpp"
#include "menu/menu.hpp"
#include "util/log.hpp"

__attribute__((constructor)) void init() {
    Log::Info("loading nightshade");

    Menu::Init();
    Interfaces::Init();
    Hooks::Init();

    int width = 0;
    int height = 0;
    Interfaces::engine->GetScreenSize(&width, &height);
    Log::Info("screen size: ({}, {})", width, height);
    Log::Info("loaded nightshade");
}

__attribute__((destructor)) void exit() {
    Log::Info("unloading nightshade");
    Menu::Shutdown();
    Hooks::Uninit();
    Log::Info("unloaded nightshade");
}
