#pragma once

#include "virtual_method.hpp"

struct EngineInterface {
    VIRTUAL_METHOD(6, GetScreenSize, void, (int *width, int *height), (this, width, height))
    // VIRTUAL_METHOD(8, GetPlayerInfo, bool, (int index, PlayerInfo *playerInfo), (this, index, playerInfo))
    VIRTUAL_METHOD(9, GetPlayerForUserID, int, (int userID), (this, userID))
    VIRTUAL_METHOD(12, GetLocalPlayer, int, (), (this))
    //VIRTUAL_METHOD(18, GetViewAngles, void, (Vector * angle), (this, angle))
    //VIRTUAL_METHOD(19, SetViewAngles, void, (Vector * angle), (this, angle))
    VIRTUAL_METHOD(20, GetMaxClients, int, (), (this))
    VIRTUAL_METHOD(26, IsInGame, bool, (), (this))
    //VIRTUAL_METHOD(37, WorldToScreenMatrix, Matrix *, (), (this))
    //VIRTUAL_METHOD(78, GetNetChannel, CNetChan *, (), (this))
};
