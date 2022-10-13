use super::*;

mod nair;
mod upsmash;
mod sidespecial;
mod nspecial;
mod uptilt;
mod upthrow;
mod fthrow;
mod upair;
mod upspecial;
mod movement;
mod opff;
mod status;

pub fn install() {
    nair::install();
    upsmash::install();
    sidespecial::install();
    nspecial::install();
    uptilt::install();
    upthrow::install();
    fthrow::install();
    upair::install();
    upspecial::install();
    movement::install();
    opff::install();
    status::install();
}
