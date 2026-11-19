use byondapi_sys::CByondXYZ;

use crate::{prelude::ByondValue, static_global::byond, Error};

/// This struct is a little weird because we're actually responsible for initializing and freeing it ourselves, unlike
/// all the rest.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ByondXYZ(CByondXYZ);

impl ByondXYZ {
    pub fn with_coords((x, y, z): (i16, i16, i16)) -> Self {
        Self(CByondXYZ { x, y, z, junk: 0 })
    }
    pub fn coordinates(&self) -> (i16, i16, i16) {
        (self.0.x, self.0.y, self.0.z)
    }
}

impl Default for ByondXYZ {
    fn default() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }
}

/// Corresponds to [`dm::block`](https://www.byond.com/docs/ref/#/proc/block)
/// Gets a list of turfs in a square zone between the two provided corners.
pub fn byond_block(corner1: ByondXYZ, corner2: ByondXYZ) -> Result<Vec<ByondValue>, Error> {
    use std::cell::RefCell;

    thread_local! {
        static BUFFER: RefCell<Vec<ByondValue>> = RefCell::new(Vec::with_capacity(1));
    }

    BUFFER.with_borrow_mut(|buff| -> Result<Vec<ByondValue>, Error> {
        let mut len = buff.capacity() as u32;
        // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
        let initial_res = unsafe {
            byond().Byond_Block(&corner1.0, &corner2.0, buff.as_mut_ptr().cast(), &mut len)
        };
        match (initial_res, len) {
            (false, 1..) => {
                buff.reserve_exact(len as usize);
                // Safety: buffer capacity is passed to byond, which makes sure it writes in-bound
                unsafe {
                    map_byond_error!(byond().Byond_Block(
                        &corner1.0,
                        &corner2.0,
                        buff.as_mut_ptr().cast(),
                        &mut len
                    ))?
                };

                // Safety: buffer should be written to at this point
                unsafe { buff.set_len(len as usize) };
                Ok(std::mem::take(buff))
            }
            (true, _) => {
                // Safety: buffer should be written to at this point
                unsafe { buff.set_len(len as usize) };
                Ok(std::mem::take(buff))
            }
            (false, 0) => Err(Error::get_last_byond_error()),
        }
    })
}

/// Corresponds to the second variation of [`dm::locate(X,Y,Z)`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds a turf at the given coordinates.
pub fn byond_locatexyz(coords: ByondXYZ) -> Result<ByondValue, Error> {
    unsafe {
        let mut output = std::mem::MaybeUninit::uninit();
        map_byond_error!(byond().Byond_LocateXYZ(&coords.0, output.as_mut_ptr()))?;
        Ok(ByondValue(output.assume_init()))
    }
}

/// Corresponds to accessing [`atom.loc`](https://www.byond.com/docs/ref/#/atom/var/loc)
/// Gets the location of the target, which will be 0,0,0 if the atom is not directly on a turf.
pub fn byond_xyz(target: &ByondValue) -> Result<ByondXYZ, Error> {
    unsafe {
        let mut output = std::mem::MaybeUninit::uninit();
        map_byond_error!(byond().Byond_XYZ(&target.0, output.as_mut_ptr()))?;
        Ok(ByondXYZ(output.assume_init()))
    }
}
