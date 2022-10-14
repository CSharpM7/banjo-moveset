use super::*;

#[acmd_script( agent = "buddy", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;  
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);


    let link_damage = 2.6;
    let finish_damage = 5.8;
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("top"), link_damage, 367, 100, 90, 0, 4.4, 0.0, 4.4, -5.6, Some(0.0), Some(11.4), Some(-5.6), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("k_bust"), link_damage, 367, 100, 90, 0, 5.0, link_damage, 0.0, 0.0, Some(8.6), Some(0.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), link_damage, 366, 100, 60, 0, 4.4, 0.0, 4.4, -5.6, Some(0.0), Some(11.4), Some(-5.6), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 3, 0, Hash40::new("k_bust"), link_damage, 110, 100, 80, 0, 3.0, -0.2, 2.0, 0.0, Some(9.4), Some(2.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 4, 0, Hash40::new("k_bust"), link_damage, 180, 100, 40, 0, 3.0, -0.2, -2.0, 0.0, Some(9.4), Some(-2.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), link_damage, 367, 100, 70, 0, 4.4, 0.0, 4.4, -5.6, Some(0.0), Some(11.4), Some(-5.6), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("k_bust"), link_damage, 367, 100, 70, 0, 5.0, link_damage, 0.0, 0.0, Some(9.4), Some(0.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), link_damage, 366, 100, 60, 0, 4.4, 0.0, 4.4, -5.6, Some(0.0), Some(11.4), Some(-5.6), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 3, 0, Hash40::new("k_bust"), link_damage, 190, 100, 40, 0, 5.0, link_damage, 0.0, 0.0, Some(9.4), Some(0.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), finish_damage, 41, 96, 0, 68, 4.8, 0.0, 4.4, -5.6, Some(0.0), Some(11.4), Some(-5.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("k_bust"), finish_damage, 41, 96, 0, 68, 5.8, link_damage, 0.0, 0.0, Some(8.6), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
    WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


pub fn install() {
    install_acmd_scripts!(
        buddy_attack_air_b_game
    );
}
