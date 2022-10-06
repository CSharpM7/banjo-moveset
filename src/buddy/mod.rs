use super::*;

mod nair;
mod upsmash;
mod sidespecial;
//mod nspecial;
mod uptilt;
mod opff;

pub fn install() {
    nair::install();
    upsmash::install();
    sidespecial::install();
    //nspecial::install();
    uptilt::install();
    opff::install();
}