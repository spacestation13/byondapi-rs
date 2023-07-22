use super::*;

impl Default for ByondValue {
    fn default() -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_Init(new_inner.as_mut_ptr());
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }
}

impl ByondValue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_ref(typ: ByondValueType, ptr: u4c) -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_InitRef(new_inner.as_mut_ptr(), typ, ptr);
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }

    pub fn new_num(f: f32) -> Self {
        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            BYOND.ByondValue_InitNum(new_inner.as_mut_ptr(), f);
            // Safety: ByondValue_Init will have initialized the new_inner.
            new_inner.assume_init()
        };

        Self(new_inner)
    }

    pub fn new_str<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        let c_str = CString::new(s.as_ref()).unwrap();

        let mut new_inner = MaybeUninit::uninit();

        let new_inner = unsafe {
            // Safety: new_inner is going to an initialization function, it will only write to the pointer.
            let success = BYOND.ByondValue_InitStr(new_inner.as_mut_ptr(), c_str.as_ptr());
            if success {
                // Safety: ByondValue_Init will have initialized the new_inner.
                new_inner.assume_init()
            } else {
                return Err(Error::get_last_byond_error());
            }
        };

        Ok(Self(new_inner))
    }
}
