use super::*;

mod nair;
mod upsmash;
mod sidespecial;
mod nspecial;
mod uptilt;
mod upthrow;
mod fthrow;
mod backair;
mod upair;
mod downair;
mod upspecial;
mod downspecial;
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
    backair::install();
    upair::install();
    downair::install();
    upspecial::install();
    downspecial::install();
    movement::install();
    opff::install();
    status::install();
}
