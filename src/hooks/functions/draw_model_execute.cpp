#include "hooks/functions.hpp"
#include "hooks/hooks.hpp"
#include "util/log.hpp"

void DrawModelExecute(
    void *thisptr, void *ctx, DrawModelState *state, ModelRenderInfo *info,
    Matrix3x4 *bone_to_world) {
    Log::Info("executed DrawModelExecute");
    reinterpret_cast<DrawModelExecuteFn>(Hooks::draw_model_execute->original_function)(
        thisptr, ctx, state, info, bone_to_world);
}
