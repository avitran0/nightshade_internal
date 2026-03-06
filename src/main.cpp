#include "util/log.hpp"

__attribute__((constructor)) void init() {
    log::Info("loading nightshade");
}

__attribute__((destructor)) void exit() {
    log::Info("unloading nightshade");
}
