use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);


static mut BEAKBOMB_ACTIVE: bool = false;
static mut BEAKBOMB_BOUNCE: i32 = 1; //0-2 for strength. 0 for a normal wall
static mut BEAKBOMB_ANGLE: f32 = 0.0;
static mut BEAKBOMB_FRAME: i32 = 0; //0-2 for strength. 0 for a normal wall
static mut BAYONET_STATE: i32 = -1; //-1 not in Breegull. 0 in breegull. 1 request attack. 2 attack
static mut BAYONET_EGGS: i32 = 0;
static mut HUD_DISPLAY_TIME: i32 = 0;
static mut HUD_DISPLAY_TIME_MAX: i32 = 90;
static mut FEATHERS_RED_COOLDOWN: i32 = 0;
static mut FEATHERS_RED_COOLDOWN_MAX: i32 = 180;
static mut FLUTTER_STATE: i32 = 0; //0 inactive, 1 active, -1 disabled

// Use a different move while using SideB in the air
unsafe fn beakbomb_cancel(fighter: &mut L2CFighterCommon){ 
    let is_guarding = fighter.is_button_on(Buttons::Guard);
    let cancel_frame = 15.0;
    let can_cancel = fighter.motion_frame() >= cancel_frame;
    if (is_guarding && can_cancel)
    {
         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

unsafe fn wonderwing_cancel(fighter: &mut L2CFighterCommon){ 
    //let status = StatusModule::status_kind(fighter.module_accessor);
    let is_guarding = fighter.is_button_on(Buttons::Guard);
    let cancel_frame = 15.0;
    let can_cancel = fighter.motion_frame() >= cancel_frame;
    if (is_guarding && can_cancel)
    {
         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}
//cancelFrame should be less than whatever frame a ShieldBonk starts on
unsafe fn sidespecial_passive(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){ 
    let side_special_wall = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL);
    if (!side_special_wall) {return;}
    let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if (has_hit_shield) {return;}
    let is_teching = ControlModule::check_button_trigger(boma,*CONTROL_PAD_BUTTON_GUARD);//fighter.is_button_on(Buttons::Guard);
    let cancel_frame = 5.0;
    let can_cancel = fighter.motion_frame() <= cancel_frame;
    if (is_teching && can_cancel)
    {
        //Clear speed
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP); 
        KineticModule::clear_speed_all(fighter.module_accessor);
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
    let side_special = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    if !side_special {return;}

    let in_Air = fighter.is_situation(*SITUATION_KIND_AIR);
    if (!in_Air)
    {
        wonderwing_cancel(fighter);
    }
	else
	{
		beakbomb_cancel(fighter);
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
        
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    //Do not update flight during hitstop
    let in_Hitstop = SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 ;
    if in_Hitstop {return;}

    //Movement
    let motion_factor = 0.375;
    let motion_offset = -0.125;
    let motion_vec = Vector3f{x: 0.0, y: motion_offset+(BEAKBOMB_ANGLE*motion_factor), z: 0.0};
    KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);

    //Drop item
    /*
    let z_drop = fighter.is_button_on(Buttons::Catch);
    if (z_drop){
        ItemModule::throw_item(fighter.module_accessor, 300.0, 3.0, 1.0, 0, true, 0.0);
    }
    */
}
unsafe fn beakbomb_check(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let sideSpecial = [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL
    ].contains(&status);
    let side_special_dash = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    let side_special_wall = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL);
    let in_Air = fighter.is_prev_situation(*SITUATION_KIND_AIR);
    
	if (status == *FIGHTER_STATUS_KIND_SPECIAL_S && FEATHERS_RED_COOLDOWN >0 && in_Air)
	{
		fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, true);
	}

    //While BEAKBOMB_ACTIVE, enable control
    if (side_special_dash && in_Air)
    {
        BEAKBOMB_FRAME +=1;
        beakbomb_checkForHit(fighter,boma);
        beakbomb_control(fighter,boma);
    }
    else if (side_special_wall)
    {
        beakbomb_bounce(fighter,boma);
    }
    //Force out of BEAKBOMB_ACTIVE if landed
    else if (!in_Air && BEAKBOMB_ACTIVE)
    {
        BEAKBOMB_ACTIVE=false;
        BEAKBOMB_FRAME=0;
        if (side_special_wall)
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
    let has_hit_foe = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT);
    let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if !(has_hit_shield 
        //|| has_hit_foe
    ) 
    {
        beakbomb_checkForFail(fighter,boma);
        return;
    }
    if (fighter.motion_frame() > 0.0) //If motion frame is 0, game crashes
    {
        let start_frame = 6.0;
        let weak_frame = 20.0;
        BEAKBOMB_BOUNCE = if (fighter.motion_frame() >= weak_frame) {1} else {2};
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, false);
        //MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, start_frame, true, true, false);
    }
}

unsafe fn beakbomb_bounce(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let start_frame = 6.0;
    if fighter.is_motion(Hash40::new("special_air_s_wall"))
    && fighter.motion_frame() < start_frame
    && fighter.motion_frame() > 0.0 {

        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            //WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            let x_bounce = match BEAKBOMB_BOUNCE{
                0=> -1.0,
                2=> -2.0,
                _=> -1.5
            };
            let y_bounce = if (BEAKBOMB_BOUNCE<1) {0.5} else {1.0};
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            SET_SPEED_EX(fighter, x_bounce, y_bounce, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			if (BEAKBOMB_BOUNCE==0){

			}
            BEAKBOMB_ACTIVE = false;
    }
	else if (fighter.motion_frame() >= 17.0 && BEAKBOMB_BOUNCE==0)
	{
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
	}
}

unsafe fn beakbomb_checkForFail(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let is_grounded = fighter.is_situation(*SITUATION_KIND_GROUND);
    let cancel_frame = 5;
    let cancel_cutoff = 25;
    let can_fail = cancel_frame < BEAKBOMB_FRAME && BEAKBOMB_FRAME < cancel_cutoff;
    if !(is_grounded) {return;}

    if (can_fail)
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
    let in_Air = fighter.is_prev_situation(*SITUATION_KIND_AIR);
    if (in_Air)
    {
        BAYONET_STATE=-1;
        return;
    }

    //println!("{}",BAYONET_STATE);
    let status = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING
    ].contains(&status) {
        if (BAYONET_STATE<=0)
        {
			BAYONET_EGGS= WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
			BAYONET_STATE=0;
            let is_csticking = ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;

            let transition_frame = 3.0;
            let can_cancel = fighter.motion_frame() <= transition_frame;
            if (is_csticking && can_cancel) {
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
    else if (status == *FIGHTER_STATUS_KIND_ATTACK_S3 && BAYONET_STATE>=0)
    {
        let transition_frame = 21.0;
        let can_cancel = fighter.motion_frame() >= transition_frame;
        if (!can_cancel) {return;}
        BAYONET_STATE=2;
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START, true);
    }
    else if (status == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START && BAYONET_STATE==2)
    {
        BAYONET_STATE=0;
		WorkModule::set_int(fighter.module_accessor,BAYONET_EGGS, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
        STOP_SE(fighter, Hash40::new("se_buddy_attackhard_s03"));
        let transition_frame = 26.0;
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("special_n_start"), false, transition_frame);
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, transition_frame, true, true, false);
    }
    else
    {
        BAYONET_STATE=-1;
    }
}

unsafe fn buddy_meter_display_update(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,RedFeather: bool) {
	EffectModule::kill_kind(boma, Hash40::new("buddy_special_s_count"), false, true);

    let FEATHERS_GOLD_COUNT = WorkModule::get_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
	//let hudZ = if (RedFeather) {25.0} else {0.0};
	let position = Vector3f::new(0.0,25.0,0.0);
	let handle = EffectModule::req_follow(
		boma,
		Hash40::new("buddy_special_s_count"),
		Hash40::new("top"),
		&position,
		&Vector3f::zero(),
		1.0,
		false,
		0,
		0,
		0,
		0,
		0,
		false,
		false,
	) as u32;

	let mut uv_offset_x = 0.0;
	let mut uv_offset_y = 0.2*(FEATHERS_GOLD_COUNT as f32);
	if (RedFeather)
	{
		uv_offset_x = -1.5;
		uv_offset_y = if (FEATHERS_RED_COOLDOWN == 0) {0.2} else {0.0};
		EffectModule::set_rgb(boma,handle, 1.0, 0.3, 0.0);
	}
	EffectModule::set_custom_uv_offset(boma, handle, &Vector2f::new(uv_offset_x,uv_offset_y), 0);
}

unsafe fn buddy_meter_display(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,RedFeather: bool){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let side_special = [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL,
        //*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL,
		*FIGHTER_STATUS_KIND_REBIRTH
    ].contains(&status);
	if (side_special && fighter.motion_frame()<=2.0)
	{
		buddy_meter_display_update(fighter,boma,RedFeather);
		HUD_DISPLAY_TIME=HUD_DISPLAY_TIME_MAX;
	}
	if (HUD_DISPLAY_TIME>0)
	{
		HUD_DISPLAY_TIME-=1;
	}
	else
	{
		EffectModule::kill_kind(boma, Hash40::new("buddy_special_s_count"), false, true);
	}
}
unsafe fn buddy_meter_controller(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let status = StatusModule::status_kind(fighter.module_accessor);
    let in_Air = fighter.is_prev_situation(*SITUATION_KIND_AIR);
	if (FEATHERS_RED_COOLDOWN>0)
	{
		FEATHERS_RED_COOLDOWN -= 1;
		if (FEATHERS_RED_COOLDOWN<=0)
		{
			FEATHERS_RED_COOLDOWN = 0;
            WorkModule::off_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
			app::FighterUtil::flash_eye_info(fighter.module_accessor);
			if (HUD_DISPLAY_TIME==0)
			{
				buddy_meter_display_update(fighter,boma,true);
				HUD_DISPLAY_TIME=HUD_DISPLAY_TIME_MAX;
			}
		}
        else if (in_Air)
        {
			WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
        }
		else
		{
            WorkModule::off_flag(boma, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
		}
	}
	buddy_meter_display(fighter,boma,in_Air);
    if (fighter.motion_frame() <= 2.0 && in_Air)
    {
        if (status == *FIGHTER_STATUS_KIND_CLIFF_CATCH
        && FEATHERS_RED_COOLDOWN > FEATHERS_RED_COOLDOWN_MAX-5)
        {
            FEATHERS_RED_COOLDOWN = 1;
        }
        else if (status == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)
        {
            FEATHERS_RED_COOLDOWN = FEATHERS_RED_COOLDOWN_MAX;
        }
	}
}

unsafe fn flutter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
	let status = StatusModule::status_kind(fighter.module_accessor);
	if fighter.is_motion(Hash40::new("jump_aerial_f2"))
	{
		let is_jumping = fighter.is_button_on(Buttons::Jump);
		let flutter_frame = 35.0;
		let speed_y = KineticModule::get_sum_speed_y(boma,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
		let can_flutter = fighter.motion_frame() < flutter_frame
		&& speed_y < 0.5
		&& FLUTTER_STATE >= 0;
		if (is_jumping && can_flutter) {
			sv_kinetic_energy!(
				set_accel,
				fighter,
				FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
				-0.05 
			);
			let speed_y_min = -0.025;
			if (speed_y<speed_y_min)
			{
				sv_kinetic_energy!(
					set_stable_speed,
					fighter,
					FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
					speed_y_min
				);
			}
			FLUTTER_STATE=1;
		}
		else if (!is_jumping && FLUTTER_STATE==1)
		{
			FLUTTER_STATE= -1;
			KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
		}
	}
	else
	{
		FLUTTER_STATE=0;
	}
}
unsafe fn breegull_fatigue(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
	let eggs_shot = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
	if (eggs_shot >= 10
	&& !fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END))
	{
		let sweatRate = if (eggs_shot<15) {25.0} else {15.0};
		let sweatSize = if (eggs_shot<15) {0.25} else {0.9};
		let modulo = fighter.motion_frame() % sweatRate;
		println!("{}",modulo);
		if (modulo<1.0)
		{
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_sweat"), Hash40::new("top"), 0, 8.5, 7.5, 0, 0, 0, sweatSize, true);
		}
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
		buddy_meter_controller(fighter,boma);
		breegull_fatigue(fighter,boma);
		//flutter(fighter,boma);
    }
}

#[fighter_reset]
fn buddy_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if fighter.kind() == *FIGHTER_KIND_BUDDY {
			FEATHERS_RED_COOLDOWN = 0;
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
    install_agent_resets!(buddy_reset);
}