use super::*;

#[acmd_script( agent = "buddy", script = "effect_specialairsdash", category = ACMD_EFFECT )]
unsafe fn buddy_special_air_s_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    /* 
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("buddy_special_s_flash1"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLW_POS(fighter, Hash40::new("buddy_special_s_flash2"), Hash40::new("k_head"), -4, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        }*/
    if (smash::app::sv_animcmd::get_value_float(lua_state,*SO_VAR_FLOAT_LR)<=0.0){
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), 2, 0, 3, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), 2, 6, 0, 0, 0, 0, 1.1, true);
            }
        }
    else if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), -2, 0, 3, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), -2, 6, 0, 0, 0, 0, 1.1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FLASH(fighter, 1, 1, 0.6, 0.3);
        }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_after_image"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        FLASH(fighter, 1, 0.3, 0, 0.4);
        }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.6, 0.3);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.4, 0, 0.05);
        }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.6, 0.3);
        }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.6, 0.3);
    }
    wait(lua_state, 2.0);
}

#[acmd_script( agent = "buddy", script = "sound_specialairsfail", category = ACMD_SOUND )]
unsafe fn buddy_special_air_s_fail_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_buddy_damage_twinkle"));
    }
}
#[acmd_script( agent = "buddy", script = "game_specialairsfail", category = ACMD_GAME )]
unsafe fn buddy_special_air_s_fail_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    frame(lua_state, 2.0);
    //if is_excute(fighter) {
        //fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, true);
    //}
}
#[acmd_script( agent = "buddy", script = "game_specialairsend", category = ACMD_GAME )]
unsafe fn buddy_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;  
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);  

    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
    }
}

#[acmd_script( agent = "buddy", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
      );
    }
}

// Uses smash_script, if you prefer to use the built-in macros instead.
#[acmd_script( agent = "buddy", script = "game_specialairsdash" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_air_s_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if is_excute(fighter) {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
          );
        //Prevents losing a gold feather
        WorkModule::add_int(boma, 1, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);

        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        JostleModule::set_status(boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 43, 64, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        
        ATTACK(fighter, 1, 0, Hash40::new("top"), 22.0, 43, 64, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(boma, 0, true);
        AttackModule::set_captured_same_time_attack(boma, 1, true);
        AttackModule::set_captured_same_time_attack_damage_mul(boma, 0, 0.25);
        AttackModule::set_captured_same_time_attack_damage_mul(boma, 1, 0.25);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.28);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        HIT_NO(fighter, 0, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 1, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 2, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 3, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 4, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 5, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 6, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 7, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 8, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 9, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 10, *HIT_STATUS_INVINCIBLE);
        HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
          );
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 43, 57, 0, 66, 3.8, 0.0, 4.2, 1.8, Some(0.0), Some(4.2), Some(3.2), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 43, 57, 0, 66, 4.2, 0.0, 9.2, 3.8, Some(0.0), Some(9.2), Some(5.4), 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(boma, 0, true);
        AttackModule::set_captured_same_time_attack(boma, 1, true);
        AttackModule::set_captured_same_time_attack_damage_mul(boma, 0, 0.25);
        AttackModule::set_captured_same_time_attack_damage_mul(boma, 1, 0.25);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.28);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_special_air_s_start_game,
        buddy_special_air_s_dash_game,
        buddy_special_air_s_dash_effect
        //buddy_special_air_s_end_game,
        //buddy_special_air_s_fail_sound
    );
}
