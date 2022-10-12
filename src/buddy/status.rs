use super::*;

use globals::*;


#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_AIR) )
    {
        if (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL))
        {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL),
                L2CValue::Bool(true)
            );
            return true.into();
        }
    }
    else if (WorkModule::get_int(fighter.module_accessor,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN) == 0)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    else
    {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    return false.into();
}

#[status_script(agent = "buddy", status = FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_AIR))
    {
        return false.into();
    }
    return original!(fighter);
}

pub fn install() {
    install_status_scripts!(
        buddy_special_s_pre,
        buddy_special_s_dash_pre
    );
}
