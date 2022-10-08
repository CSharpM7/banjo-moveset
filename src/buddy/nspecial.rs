use super::*;

#[acmd_script( agent = "buddy", script = "game_specialnupperfire" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_n_upperfire_game(fighter: &mut L2CAgentBase) {
    println!("Upper");
    //Prevents egg decay
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
}

unsafe fn tilt_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_s01"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_s02"));
    }
    if is_excute(fighter) {
        let play_vc = app::sv_math::rand(hash40("fighter"), 2);
        if play_vc == 0 {
            PLAY_SE(fighter, Hash40::new("vc_buddy_attackhard_s01"));
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_buddy_attackhard_s02"));
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_buddy_attackhard_s03"));
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_n04_01"));
    }
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_special_n04_02"));
    }
}

#[acmd_script( agent = "buddy", script = "sound_attacks3" , category = ACMD_SOUND , low_priority)]
unsafe fn buddy_attack_tilt_sound(fighter: &mut L2CAgentBase) {
    println!("Upper");
    tilt_sound(fighter);
}
#[acmd_script( agent = "buddy", script = "sound_attacks3hi" , category = ACMD_SOUND , low_priority)]
unsafe fn buddy_attack_tilt_hi_sound(fighter: &mut L2CAgentBase) {
    println!("Upper");
    tilt_sound(fighter);
}
#[acmd_script( agent = "buddy", script = "sound_attacks3lw" , category = ACMD_SOUND , low_priority)]
unsafe fn buddy_attack_tilt_lw_sound(fighter: &mut L2CAgentBase) {
    println!("Upper");
    tilt_sound(fighter);
}


pub fn install() {
    install_acmd_scripts!(
        buddy_attack_special_n_upperfire_game,
        //buddy_attack_tilt_sound,
        //buddy_attack_tilt_hi_sound,
        //buddy_attack_tilt_lw_sound
    );
}
