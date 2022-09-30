#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

pub mod buddy;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smashline::*;

use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};

#[skyline::main(name = "smashline_banjo")]
pub fn main() {
    buddy::install();
}