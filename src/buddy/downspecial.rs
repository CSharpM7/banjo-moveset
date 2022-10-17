use super::*;


unsafe fn equipBomb(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor){
    if (!ItemModule::is_have_item(fighter.module_accessor, 0)
    && fighter.is_button_on(Buttons::Special)
    )
    {
        ArticleModule::remove_exist(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_BUDDYBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BUDDYBOMB), 0, 0, false, false);
    }
}
#[acmd_script( agent = "buddy", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_LW_FLAG_GENERATE);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        equipBomb(fighter,boma);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    }
}
#[acmd_script( agent = "buddy", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_LW_FLAG_GENERATE);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        equipBomb(fighter,boma);
    }

    frame(lua_state, 45.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_special_lw_game,
        buddy_attack_special_air_lw_game
    );
}
