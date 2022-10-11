use super::*;

#[acmd_script( agent = "buddy", script = "expression_specialairsdash", category = ACMD_EXPRESSION )]
unsafe fn buddy_special_air_s_dash_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    
    if is_excute(fighter) {
        if (fighter.is_situation(*SITUATION_KIND_GROUND))
        {
            ItemModule::set_have_item_visibility(boma, false,0);
        }
        //slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        }
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
        }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
        }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
        }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_73_bodyattack1"), 0, false, 0);
        }
    wait(lua_state, 6.0);
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_73_bodyattack3"), 0, false, 0);
        }
    wait(lua_state, 2.0);
}

#[acmd_script( agent = "buddy", script = "effect_specialsstart", category = ACMD_EFFECT )]
unsafe fn buddy_special_s_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
	
	if (WorkModule::get_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN)<=1){
		return;
	}
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		if(smash::app::sv_animcmd::get_value_float(lua_state,*SO_VAR_FLOAT_LR) <= 0.0){
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_start"), Hash40::new("top"), -2, 9, -14, 0, 0, 0, 1.2, true);
		}
		else{
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_start"), Hash40::new("top"), 2, 9, -14, 0, 0, 0, 1.2, true);
		}
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_hold"), Hash40::new("virtualcenter"), 1.5, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.4, 0, 0.2);
        }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 4, 0, 0, 0, 0);
        }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.4, 0.3);
        }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.3, 0, 0.4);
        }
    wait(lua_state, 2.0);
}


#[acmd_script( agent = "buddy", script = "effect_specialairsstart", category = ACMD_EFFECT )]
unsafe fn buddy_special_air_s_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.4, 0, 0.2);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 4, 0, 0, 0, 0);
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_all"), 0, -6, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.4, 0, 0.3);
    }
}

#[acmd_script( agent = "buddy", script = "effect_specialsdash", category = ACMD_EFFECT )]
unsafe fn buddy_special_s_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 

    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("buddy_special_s_flash1"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLW_POS(fighter, Hash40::new("buddy_special_s_flash2"), Hash40::new("k_head"), -4, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);

        if(smash::app::sv_animcmd::get_value_float(lua_state,*SO_VAR_FLOAT_LR) <= 0.0){
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), 2, 0, 3, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), 2, 6, 0, 0, 0, 0, 1.1, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), -2, 0, 3, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), -2, 6, 0, 0, 0, 0, 1.1, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
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

#[acmd_script( agent = "buddy", script = "effect_specialairsdash", category = ACMD_EFFECT )]
unsafe fn buddy_special_air_s_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 

    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("buddy_special_s_flash1"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(fighter, 1, 0.3, 0);
        
        EFFECT_FLW_POS(fighter, Hash40::new("buddy_special_s_flash2"), Hash40::new("k_head"), -1, 0, -4, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }

    if (smash::app::sv_animcmd::get_value_float(lua_state,*SO_VAR_FLOAT_LR)<=0.0){
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), 2, 0, 3, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
            EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), 2, 4, 0, 0, 0, 0, 0.7, true);
        }
    }
    else if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_impact"), Hash40::new("throw"), -2, 0, 3, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), -2, 4, 0, 0, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.6, 0.3);
        EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        //buddy_meter(fighter,boma);
        
        EffectModule::enable_sync_init_pos_last(boma);
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

#[acmd_script( agent = "buddy", script = "sound_specialairsstart", category = ACMD_SOUND )]
unsafe fn buddy_special_air_s_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    
    //WorkModule::is_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 3);
        if play_vc == 0 {
            PLAY_SE(fighter, Hash40::new("vc_buddy_smash_h01_vc"));
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_s04_01"));
    }
}

#[acmd_script( agent = "buddy", script = "sound_specialairsdash", category = ACMD_SOUND )]
unsafe fn buddy_special_air_s_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_buddy_smash_h06"));
        //PLAY_STATUS(fighter, Hash40::new("vc_buddy_attackhard_l01"));
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 100);
        if play_vc == 0 {
            PLAY_SE(fighter, Hash40::new("vc_buddy_damage_twinkle"));
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        //se_common_blowaway_s,se_common_throw_02
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_s02_02"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_s02_03"));
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_s02_03"));
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_s02_03"));
    }
}
#[acmd_script( agent = "buddy", script = "sound_specialairswall", category = ACMD_SOUND )]
unsafe fn buddy_special_air_s_wall_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent; 
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_m_01"));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 2);
        if play_vc == 0 {
            PLAY_SE(fighter, Hash40::new("vc_buddy_missfoot02"));
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_m_02"));
    }
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
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 2.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);

    //6 frames of armor
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7.0);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

// Uses smash_script, if you prefer to use the built-in macros instead.
#[acmd_script( agent = "buddy", script = "game_specialairsdash" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_air_s_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if is_excute(fighter) {
        //Prevents losing a gold feather
        WorkModule::add_int(boma, 1, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);

        let shieldDamage = 4;
        //WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        JostleModule::set_status( boma, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 43, 70, 0, 66, 4.2, 0.0, 9.2, 8.8, Some(0.0), Some(9.2), Some(11.4), 1.125, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shieldDamage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 43, 70, 0, 66, 4.2, 0.0, 4.2, 2.8, None,None,None, 1.125, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, shieldDamage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        
        WorkModule::on_flag( boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_CLIFF_CHECK);

        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, /*ShieldstunMul*/ 1.5);
    }
    //Remove transition hurtbox
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    //Weaker hitbox
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 43, 57, 0, 66, 4.2, 0.0, 9.2, 8.8, Some(0.0), Some(9.2), Some(11.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, /*ShieldstunMul*/ 1.25);
    }
}

// Uses smash_script, if you prefer to use the built-in macros instead.
#[acmd_script( agent = "buddy", script = "game_specialairswall" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_air_s_wall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    
    //DamageModule::add_damage(fighter.module_accessor, 10.0,0);
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
    }
}


// Uses smash_script, if you prefer to use the built-in macros instead.
#[acmd_script( agent = "buddy", script = "game_specialsfail" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_special_s_fail_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state); 
    
    //A landing hitbox that lasts 2 frames
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 43, 57, 0, 66, 6.2, 0.0, 9.2, 5.0, None,None,None, 1.15, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_special_air_s_start_game,
        buddy_special_air_s_start_effect,
        buddy_special_air_s_start_sound,
        buddy_special_air_s_dash_expression,
        buddy_special_air_s_dash_game,
        buddy_special_air_s_dash_effect,
        buddy_special_air_s_dash_sound,
        buddy_special_air_s_wall_game,
        buddy_special_air_s_wall_sound,
        
        buddy_special_s_fail_game,
        buddy_special_s_start_effect,
        buddy_special_s_dash_effect
    );
}
