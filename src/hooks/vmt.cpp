#include "hooks/vmt.hpp"

#include <sys/mman.h>
#include <unistd.h>

#include <cstdint>

#include "interfaces/virtual_method.hpp"

VMTHook::VMTHook(int index, void *interface, void *hook_function) {
    vtable = GetVTable(interface);
    original_function = vtable[index];
    this->index = index;

    const auto page_size = sysconf(_SC_PAGESIZE);
    void *page_start =
        reinterpret_cast<void *>(reinterpret_cast<uintptr_t>(&vtable[index]) & ~(page_size - 1));

    mprotect(page_start, page_size, PROT_READ | PROT_WRITE | PROT_EXEC);
    vtable[index] = hook_function;
}

VMTHook::~VMTHook() {
    if (vtable && original_function) {
        const auto page_size = sysconf(_SC_PAGESIZE);
        void *page_start = reinterpret_cast<void *>(
            reinterpret_cast<uintptr_t>(&vtable[index]) & ~(page_size - 1));

        mprotect(page_start, page_size, PROT_READ | PROT_WRITE | PROT_EXEC);
        vtable[index] = original_function;
    }
}
