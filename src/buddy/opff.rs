use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);


static mut BEAKBOMB_ACTIVE: bool = false;
static mut BEAKBOMB_BOUNCE: i32 = 1; //0-2 for strength
static mut BEAKBOMB_ANGLE: f32 = 0.0;
static mut BAYONET_STATE: i32 = 0;
static mut WONDERWING_GIVE: bool = true;

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
//cancelFrame should be less than whatever frame a ShieldBonk starts on
unsafe fn sidespecial_passive(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){ 
    let sideSpecialWall = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL);
    if (!sideSpecialWall) {return;}
    let hasHitShield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if (hasHitShield) {return;}
    let isTeching = ControlModule::check_button_trigger(boma,*CONTROL_PAD_BUTTON_GUARD);//fighter.is_button_on(Buttons::Guard);
    let cancelFrame = 5.0;
    let canCancel = fighter.motion_frame() <= cancelFrame;
    if (isTeching && canCancel)
    {
        //Clear speed
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP); 
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP
        );
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        );
        //Tech based on y stick
        let stick_y: f32 = ControlModule::get_stick_y(boma);
        //Wall faces you the wrong way
        let passiveStatus = if (stick_y>0.75) {*FIGHTER_STATUS_KIND_WALL_JUMP} else {*FIGHTER_STATUS_KIND_PASSIVE_WALL};
        fighter.change_status_req(passiveStatus, true);
        //PostureModule::reverse_rot_y_lr(boma);
        REVERSE_LR(fighter);
        println!("{}",stick_y);
    }
}

unsafe fn sidespecial_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
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
        BEAKBOMB_BOUNCE=0;
        let stick_y: f32 = ControlModule::get_stick_y(boma);
        BEAKBOMB_ANGLE = stick_y.signum();
        if (stick_y.abs())<0.1
        {
            BEAKBOMB_ANGLE = 0.0;
        }
    }
    //Do not update flight during hitstop
    let in_Hitstop = SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 ;
    if in_Hitstop {return;}

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
    let hasHitShield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if !(hasHitShield || hasHitFoe) {
        beakbomb_checkForFail(fighter,boma);
        return;
    }
    let startFrame = 6.0;
    let weakFrame = 20.0;
    BEAKBOMB_BOUNCE = if (fighter.motion_frame() >= weakFrame) {1} else {2};
    fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, false);
    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, startFrame, true, true, false);
}

unsafe fn beakbomb_bounce(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let startFrame = 6.0;
    if fighter.is_motion(Hash40::new("special_air_s_wall"))
    && fighter.motion_frame() < startFrame {

        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            //WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            let xBounce = match BEAKBOMB_BOUNCE{
                0=> -1.5,
                2=> -2.5,
                _=> -2.0
            };
            let yBounce = if (BEAKBOMB_BOUNCE<1) {0.5} else {1.0};
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

unsafe fn buddy_meter_display(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let sideSpecial = [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL
    ].contains(&status);
    if (!sideSpecial)
    {
        EffectModule::kill_kind(boma, Hash40::new("buddy_special_s_count"), false, true);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
        sidespecial_cancel(fighter,boma);
        sidespecial_passive(fighter,boma);
        beakbomb_check(fighter,boma);
        breegull_bayonet(fighter,boma);
        buddy_meter_display(fighter,boma);
		if (WorkModule::get_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN)<=1){
			let IsGrounded = fighter.is_situation(*SITUATION_KIND_GROUND);
			if (IsGrounded){
				WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
			}
			else
			{
				WorkModule::off_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
			}
		}
		WorkModule::set_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN,10);
		if fighter.is_status(*FIGHTER_STATUS_KIND_DEAD)
		{
			WONDERWING_GIVE=true;
		}
		else if (WONDERWING_GIVE && WorkModule::get_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN)==4)
		{
			WorkModule::add_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN,1);
			WONDERWING_GIVE=false;
		}
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
}