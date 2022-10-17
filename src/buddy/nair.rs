use super::*;

#[acmd_script( agent = "buddy", script = "effect_attackairn", category = ACMD_EFFECT )]
unsafe fn buddy_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("shoulderr"), 0, -2, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("shoulderr"), 0, -2, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

#[acmd_script( agent = "buddy", script = "sound_attackairn", category = ACMD_SOUND )]
unsafe fn buddy_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 7.0);
    //50% change to play VC
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {PLAY_SE(fighter, Hash40::new("vc_buddy_attack03"));}
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
}
// Uses smash_script, if you prefer to use the built-in macros instead.
#[acmd_script( agent = "buddy", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    //Landing lag on frame 6
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    let bkb= 60;
    let kbg = 70;
    let size = 5.0;
    frame(lua_state, 8.0);
    //Start hitboxes on frame 8
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, kbg,0,bkb, size, 0.0, 8.0, 9.0, None, None, None, 1.125, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, kbg,0,bkb+10, size, 0.0, 8.0, 1.0, Some(0.0), Some(8.0), Some(3.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    //Shift main hitboxes outward after one frame
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 32, kbg,0,bkb, size, 0.0, 8.0, 15.0, None, None, None, 1.125, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 35, kbg,0,bkb+10, size, 0.0, 10.0, 1.0, Some(0.0), Some(8.0), Some(8.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    //Main hitbox lasts for 3 more frames, while the bodybox lasts for an additional frame
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 35, kbg,0,bkb+10, size-0.5, 0.0, 8.0, 1.0, Some(0.0), Some(8.0), Some(3.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    //Back hitbox on frame 15
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, kbg,0,bkb, size, 0.0, 13.0, -16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 50, kbg,0,bkb+10, size, 0.0, 10.0, -9.0, Some(0.0), Some(8.0), Some(-1.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    //Shift back hitbox inward after 3 frames
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, kbg,0,bkb, size, 0.0, 13.0, -14.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    //Back hitbox lasts for one more frame, the bodybox lasts for an additional frame
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    //Pop hitbox on frame 25, adds an extra frame of hitstun on the sweetspot 30
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 92, 32, 0, 77, size-0.25, 0.0, 7.0, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);

        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 45, kbg,0,bkb+10, size, 0.0, 10.0, 1.0, Some(0.0), Some(8.0), Some(8.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);

        AttackModule::set_add_reaction_frame_revised(boma, 0, 1.0, false);
    }
    //Pop hitbox lasts for 4 frames
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    //wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_air_n_game,
        buddy_attack_air_n_sound,
        buddy_attack_air_n_effect
    );
}
