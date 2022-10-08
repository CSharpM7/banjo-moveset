use super::*;

#[acmd_script( agent = "buddy", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn buddy_attack_tilt_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("buddy_attack100"), false, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);

        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("buddy_attack_arc"), Hash40::new("buddy_attack_arc"), Hash40::new("top"), 4, 11.5, 2, 0, 4, 115, 0.9, true, *EF_FLIP_YZ);

        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -0.25, 14, 1.5, 180, 4, 76, 0.9, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 3, 0, 0, 0, 0, 0, 0.9, true);
    }

    
}
#[acmd_script( agent = "buddy", script = "sound_attackhi3" , category = ACMD_SOUND , low_priority)]
unsafe fn buddy_attack_tilt_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    //frame(lua_state, 2.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_buddy_attack100_01"));
        PLAY_SE(fighter, Hash40::new("se_buddy_attack100_02"));
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_attack100_03"));
    }
}
#[acmd_script( agent = "buddy", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_tilt_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 

    let bkb_k = 57;
    let kbg_k = 78;
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //Banjo//
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 10.0, 70, 85, 0, 72, 3.6, 4.2, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 70, 85, 0, 72, 3.6, 4.2, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //Kazooie//
        ATTACK(fighter, 2, 0, Hash40::new("k_wingr4"), 6.8, 105, kbg_k, 0, bkb_k, 4.8, 0.0, -3.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 1.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.5);
    }
    /*
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        //0.0, -3.0, -3.0
    }
    */
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        //Banjo//
        AttackModule::clear(fighter.module_accessor, 2, false);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 6.8, 122, 85, 0, 42, 3.6, 4.2, 0.0, -0.8, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.8, 122, 85, 0, 42, 3.6, 4.2, 0.0, -0.8, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        //Kazooie//
        ATTACK(fighter, 2, 0, Hash40::new("k_wingr4"), 5.8, 105, kbg_k, 0, bkb_k, 4.8, 0.0, -3.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BUDDY_WING, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);

    }
}


pub fn install() {
    install_acmd_scripts!(
        buddy_attack_tilt_hi_effect,
        buddy_attack_tilt_hi_sound,
        buddy_attack_tilt_hi_game
    );
}
