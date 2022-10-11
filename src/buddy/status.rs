use super::*;

use globals::*;


#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    println!("{}",WorkModule::get_int(fighter.module_accessor,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN));
    return 0.into();
    /*
    if VarModule::is_flag(fighter.object(), vars::buddy::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S.into(), false.into());
        return 0.into();
    }
    else {
        original!(fighter)
    }
    */
}

pub fn install() {
    install_status_scripts!(
        buddy_special_n_pre
    );
}
