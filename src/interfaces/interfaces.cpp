#include "interfaces.hpp"

#include "interfaces/client.hpp"
#include "interfaces/convar.hpp"
#include "interfaces/engine.hpp"
#include "interfaces/entity_list.hpp"
#include "interfaces/material_system.hpp"
#include "interfaces/model_render.hpp"

bool Interfaces::Init() {
    client = GetInterface<ClientInterface>("./csgo/bin/linux64/client_client.so", "VClient018");
    entity_list = GetInterface<EntityListInterface>(
        "./csgo/bin/linux64/client_client.so", "VClientEntityList003");
    engine = GetInterface<EngineInterface>("./bin/linux64/engine_client.so", "VEngineClient014");
    material_system = GetInterface<MaterialSystemInterface>(
        "./bin/linux64/materialsystem_client.so", "VMaterialSystem080");
    convar =
        GetInterface<ConVarInterface>("./bin/linux64/materialsystem_client.so", "VEngineCvar007");
    model_render =
        GetInterface<ModelRenderInterface>("./bin/linux64/engine_client.so", "VEngineModel016");

    return true;
}
