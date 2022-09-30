use super::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

// Use a different move while using SideB in the air
unsafe fn beakbomb_check(fighter: &mut L2CFighterCommon){ 
    let status = StatusModule::status_kind(fighter.module_accessor);
    let sideSpecial = fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S);
    let inAir = fighter.is_situation(*SITUATION_KIND_AIR);
    let lastWWFrame = 17.0;
    let inLastWWFrame = fighter.motion_frame() >= lastWWFrame;
    if (sideSpecial && inAir) && inLastWWFrame//(fighter.is_button_on(Buttons::Guard)
    {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn buddy_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        beakbomb_check(fighter);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        buddy_update
    );
}