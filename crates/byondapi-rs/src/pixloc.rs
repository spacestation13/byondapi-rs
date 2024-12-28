use crate::{prelude::ByondValue, static_global::byond, Error};
use byondapi_sys::CByondPixLoc;

#[derive(Debug, Clone, Copy)]
pub struct ByondPixLoc(CByondPixLoc);

impl Default for ByondPixLoc {
    fn default() -> Self {
        Self(CByondPixLoc {
            x: 0.,
            y: 0.,
            z: 0,
            junk: 0,
        })
    }
}

/// Gets pixloc coords of an atom
pub fn byond_pixloc(src: ByondValue) -> Result<ByondPixLoc, Error> {
    let mut output = ByondPixLoc::default();

    unsafe { map_byond_error!(byond().Byond_PixLoc(&src.0, &mut output.0))? }

    Ok(output)
}

/// Gets pixloc coords of an atom based on its bounding box
pub fn byond_boundpixloc(src: ByondValue, dir: u8) -> Result<ByondPixLoc, Error> {
    let mut output = ByondPixLoc::default();

    unsafe { map_byond_error!(byond().Byond_BoundPixLoc(&src.0, dir, &mut output.0))? }

    Ok(output)
}
