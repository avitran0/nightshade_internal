#pragma once

#include "sdk/math.hpp"

struct DrawModelState {
    void *studio_hdr;
    void *studio_hw_data;
    void *renderable;
    const Matrix3x4 *model_to_world;
    void *decals;
    int draw_flags;
    int lod;
};

struct ModelRenderInfo {
    struct Model {
        char name[255];
    };

    Vector origin;
    QAngle angles;
    char _padding[0x4];
    void **pRenderable;
    const Model *pModel;
    const Matrix3x4 *pModelToWorld;
    const Matrix3x4 *pLightingOffset;
    const Vector *pLightingOrigin;
    int flags;
    int entity_index;
    int skin;
    int body;
    int hitboxset;
    void *instance;

    ModelRenderInfo() {
        pModelToWorld = nullptr;
        pLightingOffset = nullptr;
        pLightingOrigin = nullptr;
    }
};

struct ModelRenderInterface {};
