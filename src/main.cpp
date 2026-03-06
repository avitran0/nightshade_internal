#include "interfaces/interfaces.hpp"
#include "util/log.hpp"

__attribute__((constructor)) void init() {
    Log::Info("loading nightshade");

    Interfaces::InitInterfaces();
}

__attribute__((destructor)) void exit() { Log::Info("unloading nightshade"); }
