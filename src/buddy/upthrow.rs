use super::*;

#[acmd_script( agent = "buddy", script = "game_throwhi" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;  
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);

    //bkb 65->60
    if is_excute(fighter) {
        //bkb 60->55
        //kbg 60->80
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 88, 140, 0, 37, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 140, 0, 37, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.4, 361, 100, 0, 10, 4.0, 0.0, 24.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, -1, 18);
        //set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 10.0, z: 0.0});
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(boma);
        }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 0.2);
    frame(lua_state, 40.0);
    FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 56.0);
    FT_MOTION_RATE(fighter, 1);
}


pub fn install() {
    install_acmd_scripts!(
        buddy_throw_hi_game
    );
}
