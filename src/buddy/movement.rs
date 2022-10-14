use super::*;
static mut FLUTTER_ENABLED: bool = false;

#[acmd_script( agent = "buddy", script = "game_jumpaerialf1" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_jump_aerial_f1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
}
#[acmd_script( agent = "buddy", script = "game_jumpaerialf2" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_jump_aerial_f2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
}

#[acmd_script( agent = "buddy", script = "game_jumpaerialf2" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_jump_aerial_flutter_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        let is_jumping = fighter.is_button_on(Buttons::Jump);
        if (is_jumping)
        {
            FT_MOTION_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        let is_jumping = fighter.is_button_on(Buttons::Jump);
        if (is_jumping)
        {
            FT_MOTION_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}


pub fn install() {
    install_acmd_scripts!(
        buddy_jump_aerial_f1_game,
        buddy_jump_aerial_f2_game
    );
}
