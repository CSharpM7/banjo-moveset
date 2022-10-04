use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);


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

unsafe fn breegull_bayonet(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B
    ].contains(&status) {
        if (BAYONET_STATE==0)
        {
            let isCSticking = ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;

            let transitionFrame = 2.0;
            let canCancel = fighter.motion_frame() >= transitionFrame;
            if (isCSticking && canCancel) {
                println!("CStick");
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
                BAYONET_STATE=1;
            }
        }
        else
        {
            println!("CHANGE PLEASE");
            fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, true);
        }
    }
    else if (status == *FIGHTER_STATUS_KIND_ATTACK_S3 && BAYONET_STATE==1)
    {
        let transitionFrame = 21.0;
        let canCancel = fighter.motion_frame() >= transitionFrame;
        if (!canCancel) {return;}
        println!("Return");
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
        println!("Transition");
    }
    else if AttackModule::is_infliction_status(boma,*COLLISION_KIND_MASK_HIT)
    {
        BAYONET_STATE=0;
    }
}

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let sideSpecial = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
        let InAir = fighter.is_prev_situation(*SITUATION_KIND_AIR);
    
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
        if (sideSpecial)
        {
            sidespecial_cancel(fighter);
            if (InAir)
            {
                beakbomb_control(fighter,boma);
                beakbomb_checkForHit(fighter,boma);
            }
        }
        else
        {
            breegull_bayonet(fighter,boma);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
}