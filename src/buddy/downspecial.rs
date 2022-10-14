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

//Wrong agent hook but we'll keep this here I guess
#[acmd_script( agent = "buddy_bomb", script = "game_born" , category = ACMD_GAME , low_priority)]
unsafe fn bomb_born(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if(is_excute(fighter))
    {
        QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
        //ATTACK_LR_CHECK_POS
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 72, 46, 0, 72, 9.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);

        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    wait(lua_state, 1.0);
    if(is_excute(fighter)){
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 72, 46, 0, 72, 9.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);

        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}
#[acmd_script( agent = "buddy_bomb", script = "game_throw" , category = ACMD_GAME , low_priority)]
unsafe fn bomb_throw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    if(is_excute(fighter))
    {
        //ATTACK_LR_CHECK_POS
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 45, 100, 30, 0, 1.4, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BOMB);

        AttackModule::enable_safe_pos(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        buddy_attack_special_lw_game,
        buddy_attack_special_air_lw_game,
        bomb_born,
        bomb_throw
    );
}
