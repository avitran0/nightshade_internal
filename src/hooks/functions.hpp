#pragma once

#include <SDL_video.h>
#include <SDL_events.h>

#include "interfaces/model_render.hpp"
#include "sdk/math.hpp"

using SwapWindowFn = void (*)(SDL_Window *);

void SwapWindowHook(SDL_Window *window);

using PollEventFn = void (*)(SDL_Event *);

void PollEvent(SDL_Event *event);

using DrawModelExecuteFn = void (*)(
    void *thisptr, void *ctx, DrawModelState *state, ModelRenderInfo *info,
    Matrix3x4 *bone_to_world);

void DrawModelExecute(
    void *thisptr, void *ctx, DrawModelState *state, ModelRenderInfo *info,
    Matrix3x4 *bone_to_world);
