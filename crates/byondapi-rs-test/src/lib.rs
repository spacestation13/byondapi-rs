use byondapi::{typecheck_trait::ByondTypeCheck, value::ByondValue};

#[no_mangle]
/// # Safety
/// It's fucked
pub unsafe extern "cdecl" fn test(argc: byondapi_sys::u4c, argv: *const ByondValue) -> ByondValue {
    if argc == 1 {
        if !argv.is_null() {
            let value = argv.read();

            if value.is_num() {
                let value: Result<f32, _> = value.try_into();
                if let Ok(value) = value {
                    (value * 2.).into()
                } else {
                    (-4.0).into()
                }
            } else {
                (-3.0).into()
            }
        } else {
            (-2.0).into()
        }
    } else {
        (-1.0).into()
    }
}
