#pragma once

#include "interfaces/virtual_method.hpp"
#include "sdk/convar.hpp"

struct ConVarInterface {
    VIRTUAL_METHOD(15, Get, ConVar *, (const char *name), (this, name))
};
