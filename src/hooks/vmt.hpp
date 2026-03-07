#pragma once

struct VMTHook {
    void **vtable;
    void *original_function;
    int index;

    VMTHook(int index, void *interface, void *hook_function);
    ~VMTHook();
};
