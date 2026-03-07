#pragma once

#include "sdk/base_entity.hpp"
#include "sdk/base_handle.hpp"
#include "virtual_method.hpp"

struct EntityListInterface {
    VIRTUAL_METHOD(3, GetClientEntity, BaseEntity *, (int index), (this, index))
    VIRTUAL_METHOD(
        4, GetClientEntityFromHandle, BaseEntity *, (BaseHandle * handle), (this, handle))
    VIRTUAL_METHOD(6, GetHighestEntityIndex, int, (), (this))
};
