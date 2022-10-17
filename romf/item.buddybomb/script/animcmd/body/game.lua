game_Throw = function ()
    if sv_animcmd.is_excute() then
        local f1_local0, f1_local1, f1_local2 = nil
        sv_animcmd.ATTACK(0, 0, 13402447818, 0.10000000149011612, 45, 100, 30, 0, 1.399999976158142, 0, 0, 0, f1_local0, f1_local1, f1_local2, 0, 1, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_SPEED, false, 0, 0, 0, true, false, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE, COLLISION_PART_MASK_ALL, false, 92925133491, ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_NONE, ATTACK_REGION_BOMB)
        AttackModule.enable_safe_pos()
    end
    return 
end

game_Born = function ()
    if sv_animcmd.is_excute() then
        sv_animcmd:QUAKE()
        local f2_local0, f2_local1, f2_local2 = nil
        sv_animcmd.ATTACK(0, 0, 13402447818, 6.5, 72, 46, 0, 72, 9.800000190734863, 0, 0, 0, f2_local0, f2_local1, f2_local2, 1, 1, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_SPEED, false, 0, 0, 0, true, true, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, true, 84803683138, ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_BOMB, ATTACK_REGION_BOMB)
        ControlModule.set_rumble(54855258119, 0, false)
    end
    sv_animcmd.wait(1)
    if sv_animcmd.is_excute() then
        local f2_local0, f2_local1, f2_local2 = nil
        sv_animcmd.ATTACK(0, 0, 13402447818, 6.5, 72, 46, 0, 72, 9.800000190734863, 0, 0, 0, f2_local0, f2_local1, f2_local2, 1, 1, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_SPEED, false, 0, 0, 0, true, true, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, 84803683138, ATTACK_SOUND_LEVEL_L, COLLISION_SOUND_ATTR_BOMB, ATTACK_REGION_BOMB)
        ControlModule.set_rumble(72656470004, 0, false)
    end
    return 
end

return 
