use byondapi_sys::CByondXYZ;

use crate::{prelude::ByondValue, static_global::byond, Error};

/// This struct is a little weird because we're actually responsible for initializing and freeing it ourselves, unlike
/// all the rest.
#[derive(Debug, Clone, Copy)]
pub struct ByondXYZ(CByondXYZ);

impl ByondXYZ {
    pub fn new() -> Self {
        Self(CByondXYZ {
            x: 0,
            y: 0,
            z: 0,
            junk: 0,
        })
    }

    pub fn with_coords((x, y, z): (i16, i16, i16)) -> Self {
        Self(CByondXYZ { x, y, z, junk: 0 })
    }
    pub fn coordinates(&self) -> (i16, i16, i16) {
        (self.0.x, self.0.y, self.0.z)
    }
}

impl Default for ByondXYZ {
    fn default() -> Self {
        Self::new()
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

/// Corresponds to the first variation of [`dm::locate(Type) in Container`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds an object prototype or tag within the haystack, usually used for finding objects within a turf/area/etc
pub fn byond_locatein(needle: &ByondValue, haystack: &ByondValue) -> Result<ByondValue, Error> {
    let mut output = ByondValue::new();

    // Safety: needle, haystack, and output must be initialized, we take care of this.
    unsafe { map_byond_error!(byond().Byond_LocateIn(&needle.0, &haystack.0, &mut output.0))? };

    Ok(output)
}

/// Corresponds to the third and forth variation of [`dm::locate(Tag/TextRef)`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds an object prototype or tag within the world.
pub fn byond_locateby(target: &ByondValue) -> Result<ByondValue, Error> {
    let mut output = ByondValue::new();

    // Safety: target and output must be initialized, we take care of this.
    unsafe {
        map_byond_error!(byond().Byond_LocateIn(&target.0, std::ptr::null(), &mut output.0))?
    };

    Ok(output)
}

/// Corresponds to the second variation of [`dm::locate(X,Y,Z)`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds a turf at the given coordinates.
pub fn byond_locatexyz(coords: ByondXYZ) -> Result<ByondValue, Error> {
    let mut output = ByondValue::new();

    // Safety: coords and output must be initialized, we take care of this.
    unsafe { map_byond_error!(byond().Byond_LocateXYZ(&coords.0, &mut output.0))? };

    Ok(output)
}

/// Corresponds to accessing [`atom.loc`](https://www.byond.com/docs/ref/#/atom/var/loc)
/// Gets the location of the target, which will be 0,0,0 if the atom is not directly on a turf.
pub fn byond_xyz(target: &ByondValue) -> Result<ByondXYZ, Error> {
    let mut output = ByondXYZ::new();

    // Safety: target and output must be initialized, we take care of this.
    unsafe { map_byond_error!(byond().Byond_XYZ(&target.0, &mut output.0))? };

    Ok(output)
}
