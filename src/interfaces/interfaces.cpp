#include "interfaces.hpp"

#include "interfaces/client.hpp"
#include "interfaces/engine.hpp"
#include "interfaces/entity_list.hpp"

bool Interfaces::InitInterfaces() {
    client = GetInterface<Client>("./csgo/bin/linux64/client_client.so", "VClient018");
    entity_list =
        GetInterface<EntityList>("./csgo/bin/linux64/client_client.so", "VClientEntityList003");
    engine = GetInterface<Engine>("./bin/linux64/engine_client.so", "VEngineClient014");

    return true;
}
