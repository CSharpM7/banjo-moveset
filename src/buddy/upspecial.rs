use super::*;

#[acmd_script( agent = "buddy", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if is_excute(fighter) {
        ArticleModule::shoot_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PAD, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter,Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_HI_FLAG_RESET_CONTROL);
        notify_event_msc_cmd!(fighter,Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_special_air_hi_game
    );
}
