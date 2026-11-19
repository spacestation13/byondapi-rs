use crate::{prelude::ByondValue, static_global::byond, Error};

/// Gets pixloc coords of an atom in the form of (x, y, z)
pub fn byond_pixloc(src: ByondValue) -> Result<(f32, f32, i16), Error> {
    unsafe {
        let mut output = std::mem::MaybeUninit::uninit();
        map_byond_error!(byond().Byond_PixLoc(&src.0, output.as_mut_ptr()))?;
        let res = output.assume_init();
        Ok((res.x, res.y, res.z))
    }
}

/// Gets pixloc coords of an atom based on its bounding box in the form of (x, y, z)
pub fn byond_boundpixloc(src: ByondValue, dir: u8) -> Result<(f32, f32, i16), Error> {
    unsafe {
        let mut output = std::mem::MaybeUninit::uninit();
        map_byond_error!(byond().Byond_BoundPixLoc(&src.0, dir, output.as_mut_ptr()))?;
        let res = output.assume_init();
        Ok((res.x, res.y, res.z))
    }
}
