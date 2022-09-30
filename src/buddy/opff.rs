use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

// Use a different move while using SideB in the air
unsafe fn beakbomb_check(fighter: &mut L2CFighterCommon){ 
    let status = StatusModule::status_kind(fighter.module_accessor);
    let sideSpecial = fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S);
    let inAir = fighter.is_situation(*SITUATION_KIND_AIR);
    let lastWWFrame = 17.0;
    let inLastWWFrame = fighter.motion_frame() >= lastWWFrame;
    //if (sideSpecial && inAir) && inLastWWFrame//(fighter.is_button_on(Buttons::Guard)
    {
        //println!("It'sa me, Mario, wahoooooooo!");
        //fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

unsafe fn wonderwing_cancel(fighter: &mut L2CFighterCommon){ 
    //let status = StatusModule::status_kind(fighter.module_accessor);
    let isGuarding = fighter.is_button_on(Buttons::Guard);
    let sideSpecial = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    let inAir = fighter.is_situation(*SITUATION_KIND_AIR);
    let cancelFrame = 10.0;
    let canCancel = fighter.motion_frame() >= cancelFrame;
    if (sideSpecial && isGuarding && canCancel)
    {
        if (inAir)
        {
            fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, true);
        }
        else
        {
            fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, true);
        }
    }
    //let lastWWFrame = 17.0;
    //let inLastWWFrame = fighter.motion_frame() >= lastWWFrame;
    //if (sideSpecial && inAir) && inLastWWFrame//(fighter.is_button_on(Buttons::Guard)
    {
        //println!("It'sa me, Mario, wahoooooooo!");
        //fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        beakbomb_check(fighter);
        wonderwing_cancel(fighter);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
}