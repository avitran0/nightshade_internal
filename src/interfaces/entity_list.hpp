#pragma once

#include "sdk/c_base_entity.hpp"
#include "sdk/c_base_handle.hpp"
#include "virtual_method.hpp"

struct EntityListInterface {
    VIRTUAL_METHOD(3, GetClientEntity, CBaseEntity *, (int index), (this, index))
    VIRTUAL_METHOD(
        4, GetClientEntityFromHandle, CBaseEntity *, (CBaseHandle * handle), (this, handle))
    VIRTUAL_METHOD(6, GetHighestEntityIndex, int, (), (this))
};
