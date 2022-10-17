use super::*;

unsafe fn will_Bayonet(fighter: &mut L2CAgentBase) -> bool
{
    let is_csticking = ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;
    if (is_csticking) {
        println!("CStick");
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
        return true;
    }
    return false;
}

#[acmd_script( agent = "buddy", script = "game_specialnupperfire" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_n_upperfire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    for _ in 0..4
    {
        if is_excute(fighter) && will_Bayonet(fighter){
            return;
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
    }
}
#[acmd_script( agent = "buddy", script = "game_specialnfire2" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_n_fire2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    for _ in 0..4
    {
        if is_excute(fighter) && will_Bayonet(fighter){
            return;
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
    }
}


#[acmd_script( agent = "buddy", script = "game_specialnreturn" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_special_n_return_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        //ArticleModule::generate_article(boma, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER);

        let transition_frame = 26.0;
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("special_n_start"), false, transition_frame);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_special_n_upperfire_game,
        buddy_attack_special_n_fire2_game,
        buddy_attack_special_n_return_game
    );
}
