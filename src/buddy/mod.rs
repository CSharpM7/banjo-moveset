use super::*;

mod nair;
mod upsmash;
mod sidespecial;
mod opff;

pub fn install() {
    nair::install();
    upsmash::install();
    sidespecial::install();
    opff::install();
}