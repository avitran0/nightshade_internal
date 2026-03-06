#pragma once

#include <dlfcn.h>

#include <cstdint>
#include <string>

#include "interfaces/client.hpp"
#include "interfaces/convar.hpp"
#include "interfaces/engine.hpp"
#include "interfaces/entity_list.hpp"
#include "interfaces/material_system.hpp"
#include "util/log.hpp"

namespace Interfaces {
    inline ClientInterface *client;
    inline EntityListInterface *entity_list;
    inline EngineInterface *engine;
    inline MaterialSystemInterface *material_system;
    inline ConVarInterface *convar;

    bool InitInterfaces();

    using InterfaceCreateFn = void *(*)();
    struct InterfaceRegistration {
        InterfaceCreateFn create_fn;
        const char *name;
        InterfaceRegistration *next;
    };

    template <typename T>
    T *GetInterface(const std::string &library, const std::string &interface_name) {
        const auto handle = dlopen(library.c_str(), RTLD_NOLOAD | RTLD_NOW | RTLD_LOCAL);
        if (!handle) {
            Log::Error("Failed to open {}", library);
            return nullptr;
        }

        using CreateInterfaceFn = void *(*)(const char *, int *);
        auto create_interface =
            reinterpret_cast<CreateInterfaceFn>(dlsym(handle, "CreateInterface"));
        if (!create_interface) {
            Log::Error("Failed to find CreateInterface in {}", library);
            dlclose(handle);
            return nullptr;
        }

        T *interface = reinterpret_cast<T *>(create_interface(interface_name.c_str(), nullptr));
        if (interface) {
            Log::Info(
                "Found interface {} at 0x{:x}", interface_name,
                reinterpret_cast<uintptr_t>(interface));
        } else {
            Log::Error("Failed to find interface {} in {}", interface_name, library);
        }

        dlclose(handle);
        return interface;
    }
}  // namespace Interfaces
