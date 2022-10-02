use super::*;

#[acmd_script( agent = "buddy", script = "effect_attackhi4", category = ACMD_EFFECT )]
unsafe fn buddy_attack_smash_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 0, 1.5, -90, 0, 0, 1.7, true, *EF_FLIP_YZ);
    LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 5, 1, -90, 0, 0, 1.2, true);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "buddy", script = "sound_attackhi4", category = ACMD_SOUND )]
unsafe fn buddy_attack_smash_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_h01"));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_buddy_attack05"));
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_buddy_attackhard_h02"));
    }
}
// Uses smash_script, if you prefer to use the built-in macros instead.
#[acmd_script( agent = "buddy", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_smash_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 14, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 15, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 16, *HIT_STATUS_NORMAL);
        HIT_NO(fighter, 17, *HIT_STATUS_NORMAL);
        FighterAreaModuleImpl::enable_fix_jostle_area( boma, 4.0, 3.0);
    }
    //Charge on frame 4
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    //Launcher hitbox, chains into main hitbox
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 115, 100, 155, 0, 5.0, 0.0, 3.0, 6.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_KICK);
    }
    //Original kbg: 110, original bkb: 42
    let kbg = 88;
    let bkb = 40;
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("legr"), 15.0, 86, kbg, 0, bkb, 4.4, 1.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footr"), 15.0, 86, kbg, 0, bkb, 4.6, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("legl"), 15.0, 86, kbg, 0, bkb, 4.4, 2.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    //Launch box lasts for 2 frames
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        HIT_NO(fighter, 14, *HIT_STATUS_OFF);
        HIT_NO(fighter, 15, *HIT_STATUS_OFF);
        HIT_NO(fighter, 16, *HIT_STATUS_OFF);
        HIT_NO(fighter, 17, *HIT_STATUS_OFF);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_smash_hi_game,
        buddy_attack_smash_hi_sound,
        buddy_attack_smash_hi_effect
    );
}
