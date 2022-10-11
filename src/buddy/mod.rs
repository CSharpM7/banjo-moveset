use super::*;

mod nair;
mod upsmash;
mod sidespecial;
mod nspecial;
mod uptilt;
mod upthrow;
mod fthrow;
mod upair;
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
    opff::install();
    status::install();
}
