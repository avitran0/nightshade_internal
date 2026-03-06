#include "interfaces/interfaces.hpp"
#include "util/log.hpp"

__attribute__((constructor)) void init() {
    Log::Info("loading nightshade");

    Interfaces::InitInterfaces();

    int width = 0;
    int height = 0;
    Interfaces::engine->GetScreenSize(&width, &height);
    Log::Info("screen size: ({}, {})", width, height);
}

__attribute__((destructor)) void exit() { Log::Info("unloading nightshade"); }
