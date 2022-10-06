use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);


static mut BEAKBOMB_ACTIVE: bool = false;
static mut BEAKBOMB_WEAK_BOUNCE: bool = false;
static mut BEAKBOMB_ANGLE: f32 = 0.0;
static mut BAYONET_STATE: i32 = 0;

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
    if (!BEAKBOMB_ACTIVE)
    {
        BEAKBOMB_ACTIVE=true;
        let stick_y: f32 = ControlModule::get_stick_y(boma);
        BEAKBOMB_ANGLE = stick_y.signum();
        if (stick_y.abs())<0.1
        {
            BEAKBOMB_ANGLE = 0.0;
        }
    }
    //Do not update flight during hitstun
    if AttackModule::is_infliction_status(boma,*COLLISION_KIND_MASK_HIT) {return;}

    //Movement
    let motion_factor = 0.375;
    let motion_offset = -0.125;
    let motion_vec = Vector3f{x: 0.0, y: motion_offset+(BEAKBOMB_ANGLE*motion_factor), z: 0.0};
    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);

    //Drop item
    let zDrop = fighter.is_button_on(Buttons::Catch);
    if (zDrop){
        ItemModule::throw_item(fighter.module_accessor, 300.0, 3.0, 1.0, 0, true, 0.0);
    }
}
unsafe fn beakbomb_check(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let sideSpecial = [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL
    ].contains(&status);
    let sideSpecialDash = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    let sideSpecialWall = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL);
    let InAir = fighter.is_prev_situation(*SITUATION_KIND_AIR);

    //While BEAKBOMB_ACTIVE, enable control
    if (sideSpecialDash && InAir)
    {
        beakbomb_checkForHit(fighter,boma);
        beakbomb_control(fighter,boma);
    }
    else if (sideSpecialWall)
    {
        beakbomb_bounce(fighter,boma);
    }
    //Force out of BEAKBOMB_ACTIVE if landed
    else if (!InAir && BEAKBOMB_ACTIVE)
    {
        BEAKBOMB_ACTIVE=false;
        if (sideSpecialWall)
        {
            fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, false);
        }
    }
    //If out of SideSpecial (Dash), then set BEAKBOMB_ACTIVE to false
    else if !(sideSpecial)
    {
        BEAKBOMB_ACTIVE=false;
    }

}

unsafe fn beakbomb_checkForHit(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let hasHitFoe = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT);
    let hasHitSheild = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if !(hasHitSheild || hasHitFoe) {
        beakbomb_checkForFail(fighter,boma);
        return;
    }
    let startFrame = 6.0;
    let weakFrame = 20.0;
    BEAKBOMB_WEAK_BOUNCE = fighter.motion_frame() >= weakFrame;
    fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, false);
    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, startFrame, true, true, false);
}

unsafe fn beakbomb_bounce(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let startFrame = 6.0;
    if fighter.is_motion(Hash40::new("special_air_s_wall"))
    && fighter.motion_frame() < startFrame {

        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            //WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            let xBounce = if (BEAKBOMB_WEAK_BOUNCE) {-1.5} else {-2.5};
            let yBounce = if (BEAKBOMB_WEAK_BOUNCE) {0.5} else {1.0};
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            SET_SPEED_EX(fighter, xBounce, yBounce, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe fn beakbomb_checkForFail(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let IsGrounded = fighter.is_situation(*SITUATION_KIND_GROUND);
    let cancelFrame = 5.0;
    let cancelCutoff = 25.0;
    let canFail = cancelFrame < fighter.motion_frame() && fighter.motion_frame() < cancelCutoff;
    if !(IsGrounded) {return;}

    if (canFail)
    {
        DamageModule::add_damage(fighter.module_accessor, 10.0,0);
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, false);
        PLAY_SE(fighter, Hash40::new("vc_buddy_missfoot01"));
    }
    else
    {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, false);
    }

}

unsafe fn breegull_bayonet(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let InAir = fighter.is_prev_situation(*SITUATION_KIND_AIR);
    if (InAir)
    {
        BAYONET_STATE=0;
        return;
    }

    //println!("{}",BAYONET_STATE);
    let status = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B
    ].contains(&status) {
        if (BAYONET_STATE==0)
        {
            let isCSticking = ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;

            let transitionFrame = 3.0;
            let canCancel = fighter.motion_frame() <= transitionFrame;
            if (isCSticking && canCancel) {
                println!("CStick");
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
                BAYONET_STATE=1;
            }
        }
        else //Force change if previous block did not work
        {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
        }
    }
    else if (status == *FIGHTER_STATUS_KIND_ATTACK_S3 && BAYONET_STATE==1)
    {
        let transitionFrame = 21.0;
        let canCancel = fighter.motion_frame() >= transitionFrame;
        if (!canCancel) {return;}
        BAYONET_STATE=2;
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START, true);
    }
    else if (status == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START && BAYONET_STATE==2)
    {
        BAYONET_STATE=0;
        STOP_SE(fighter, Hash40::new("se_buddy_attackhard_s03"));
        let transitionFrame = 26.0;
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("special_n_start"), false, transitionFrame);
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, transitionFrame, true, true, false);
    }
    //If Breegull was cancelled (in)voluntarily, revert state
    /*
    else if [
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END,
        *FIGHTER_STATUS_KIND_DAMAGE
    ].contains(&status)*/
    else
    {
        BAYONET_STATE=0;
    }
}

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
        let isGuarding = fighter.is_button_on(Buttons::Guard);
        if (isGuarding)
        {
            println!(
                "[Fighter Hook]\nStart: {}\nShoot: {}\nWalkF: {}\nWalkB: {}\nEND: {}",
                *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START,
                *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
                *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
                *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
                *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END
            );

        }
        sidespecial_cancel(fighter);
        beakbomb_check(fighter,boma);
        breegull_bayonet(fighter,boma);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
}