use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

// Use a different move while using SideB in the air
unsafe fn beakbomb_cancel(fighter: &mut L2CFighterCommon){ 
    let isGuarding = fighter.is_button_on(Buttons::Guard);
    let cancelFrame = 5.0;
    let canCancel = fighter.motion_frame() >= cancelFrame;
    if (isGuarding && canCancel)
    {
         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

unsafe fn wonderwing_cancel(fighter: &mut L2CFighterCommon){ 
    //let status = StatusModule::status_kind(fighter.module_accessor);
    let isGuarding = fighter.is_button_on(Buttons::Guard);
    let cancelFrame = 15.0;
    let canCancel = fighter.motion_frame() >= cancelFrame;
    if (isGuarding && canCancel)
    {
         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

unsafe fn sidespecial_cancel(fighter: &mut L2CFighterCommon){
    let sideSpecial = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    if !sideSpecial {return;}
    let inAir = fighter.is_situation(*SITUATION_KIND_AIR);
    if (!inAir)
    {
        wonderwing_cancel(fighter);
    }
}

unsafe fn beakbomb_control(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    //Do not update flight during hitstun
    if AttackModule::is_infliction_status(boma,*COLLISION_KIND_MASK_HIT) {return;}

    //Movement
    let mut motion_y = 0.5;
    let motion_offset = -0.25;
    let stick_y: f32 = ControlModule::get_stick_y(boma);
    if (stick_y.abs())<0.1
    {
        motion_y = 0.0;
    }
    else
    {
        motion_y*= stick_y.signum();
    }

    let motion_vec = Vector3f{x: 0.0, y: motion_offset+(motion_y), z: 0.0};
    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);

    //Drop item
    let zDrop = fighter.is_button_on(Buttons::Catch);
    if (zDrop){
        ItemModule::throw_item(fighter.module_accessor, 300.0, 3.0, 1.0, 0, true, 0.0);
    }
}
unsafe fn beakbomb_checkForHit(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let hasHitFoe = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT);
    let hasHitSheild = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if !(hasHitSheild || hasHitFoe) {
        beakbomb_checkForFail(fighter,boma);
        return;
    }
    
}

unsafe fn beakbomb_checkForFail(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let IsGrounded = fighter.is_situation(*SITUATION_KIND_GROUND);
    let cancelFrame = 5.0;
    let cancelCutoff = 25.0;
    let canFail = cancelFrame < fighter.motion_frame() && fighter.motion_frame() < cancelCutoff;
    if !(IsGrounded && canFail) {return;}

    DamageModule::add_damage(fighter.module_accessor, 10.0,0);
    fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, true);
    PLAY_SE(fighter, Hash40::new("vc_buddy_missfoot01"));
}

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let sideSpecial = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
        let InAir = fighter.is_prev_situation(*SITUATION_KIND_AIR);
    
        if (sideSpecial)
        {
            sidespecial_cancel(fighter);
            if (InAir)
            {
                beakbomb_control(fighter,boma);
                beakbomb_checkForHit(fighter,boma);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
}