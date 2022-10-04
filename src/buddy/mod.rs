use super::*;

mod nair;
mod upsmash;
mod sidespecial;
//mod nspecial;
mod opff;

pub fn install() {
    nair::install();
    upsmash::install();
    sidespecial::install();
    //nspecial::install();
    opff::install();
}