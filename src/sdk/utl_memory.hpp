#pragma once

#include <cstdlib>

template <typename T>
struct UtlMemory {
    inline T &operator[](int i) { return memory[i]; }

    T *memory = reinterpret_cast<T *>(malloc(allocation_count * sizeof(T)));
    int allocation_count = 0;
    int grow_size = 0;
};
