use byondapi_sys::CByondXYZ;

use crate::{
    prelude::{ByondValue, ByondValueList},
    static_global::BYOND,
    Error,
};

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
}

impl Default for ByondXYZ {
    fn default() -> Self {
        Self::new()
    }
}

/// Corresponds to [`dm::block`](https://www.byond.com/docs/ref/#/proc/block)
/// Gets a list of turfs in a square zone between the two provided corners.
pub fn byond_block(corner1: ByondXYZ, corner2: ByondXYZ) -> Result<ByondValueList, Error> {
    match BYOND.get_version() {
        (515.., 1611..) => {}
        _ => return Err(Error::NotAvailableForThisByondVersion),
    }

    let mut output_list = ByondValueList::new();

    // Safety: output_list must be initialized, which we ensure
    unsafe { map_byond_error!(BYOND.Byond_Block(&corner1.0, &corner2.0, &mut output_list.0))? };

    Ok(output_list)
}

/// Corresponds to [`dm::length`](https://www.byond.com/docs/ref/#/proc/length)
/// Gives the length of a list or the length in bytes of a string.
pub fn byond_length(src: &ByondValue) -> Result<ByondValue, Error> {
    match BYOND.get_version() {
        (515.., 1611..) => {}
        _ => return Err(Error::NotAvailableForThisByondVersion),
    }

    let mut output = ByondValue::new();
    // Safety: src and output must be initialized, we take care of this.
    unsafe { map_byond_error!(BYOND.Byond_Length(&src.0, &mut output.0))? };
    Ok(output)
}

/// Corresponds to the first variation of [`dm::locate(Type) in Container`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds an object prototype or tag within the haystack, usually used for finding objects within a turf/area/etc
pub fn byond_locatein(needle: &ByondValue, haystack: &ByondValue) -> Result<ByondValue, Error> {
    match BYOND.get_version() {
        (515.., 1611..) => {}
        _ => return Err(Error::NotAvailableForThisByondVersion),
    }

    let mut output = ByondValue::new();

    // Safety: needle, haystack, and output must be initialized, we take care of this.
    unsafe { map_byond_error!(BYOND.Byond_LocateIn(&needle.0, &haystack.0, &mut output.0))? };

    Ok(output)
}

/// Corresponds to the third and forth variation of [`dm::locate(Tag/TextRef)`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds an object prototype or tag within the world.
pub fn byond_locateby(target: &ByondValue) -> Result<ByondValue, Error> {
    match BYOND.get_version() {
        (515.., 1611..) => {}
        _ => return Err(Error::NotAvailableForThisByondVersion),
    }

    let mut output = ByondValue::new();

    // Safety: target and output must be initialized, we take care of this.
    unsafe { map_byond_error!(BYOND.Byond_LocateIn(&target.0, std::ptr::null(), &mut output.0))? };

    Ok(output)
}

/// Corresponds to the second variation of [`dm::locate(X,Y,Z)`](https://www.byond.com/docs/ref/#/proc/locate)
/// Finds a turf at the given coordinates.
pub fn byond_locatexyz(coords: ByondXYZ) -> Result<ByondValue, Error> {
    match BYOND.get_version() {
        (515.., 1611..) => {}
        _ => return Err(Error::NotAvailableForThisByondVersion),
    }

    let mut output = ByondValue::new();

    // Safety: coords and output must be initialized, we take care of this.
    unsafe { map_byond_error!(BYOND.Byond_LocateXYZ(&coords.0, &mut output.0))? };

    Ok(output)
}

/// Corresponds to accessing [`atom.loc`](https://www.byond.com/docs/ref/#/atom/var/loc)
/// Gets the location of the target, which will be 0,0,0 if the atom is not directly on a turf.
pub fn byond_xyz(target: &ByondValue) -> Result<ByondXYZ, Error> {
    match BYOND.get_version() {
        (515.., 1611..) => {}
        _ => return Err(Error::NotAvailableForThisByondVersion),
    }

    let mut output = ByondXYZ::new();

    // Safety: target and output must be initialized, we take care of this.
    unsafe { map_byond_error!(BYOND.Byond_XYZ(&target.0, &mut output.0))? };

    Ok(output)
}
