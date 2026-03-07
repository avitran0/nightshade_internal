#pragma once

#include "sdk/utl_memory.hpp"

template <typename T>
struct CUtlVector {
    inline T &operator[](int i) { return m_Memory[i]; }

    inline int Length() const { return size; }

    UtlMemory<T> m_Memory;
    int size = 0;
    T *elements;
};
