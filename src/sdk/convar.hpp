#pragma once

#include "interfaces/virtual_method.hpp"

struct ConVar {
    VIRTUAL_METHOD(15, GetFloat, float, (), (this))
    VIRTUAL_METHOD(16, GetInt, int, (), (this))
    inline bool GetBool() { return GetInt(); }
};
