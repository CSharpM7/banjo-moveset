use super::*;

use globals::*;

/*
#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn buddy_special_n_pre(fighter: &mut L2CFighterCommon) -> smash::lib:L2CValue{
    println!("{}",WorkModule::get_int(fighter.module_accessor,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN));
    if VarModule::is_flag(fighter.object(), vars::buddy::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        fighter.change_status(FIGHTER_buddy_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
        return 0.into();
    }
    else {
        original!(fighter)
    }
}
*/
pub fn install() {
    install_status_scripts!(
//        buddy_special_n_pre
    );
}
