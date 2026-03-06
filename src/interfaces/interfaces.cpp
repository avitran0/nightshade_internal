#include "interfaces.hpp"

#include "engine.hpp"

bool Interfaces::InitInterfaces() {
    engine_interface = GetInterface<EngineInterface>("engine_client.so", "VEngineClient014");

    return true;
}
