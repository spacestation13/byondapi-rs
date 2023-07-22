use byondapi::{typecheck_trait::ByondTypeCheck, value::ByondValue};

#[no_mangle]
/// # Safety
/// It's fucked
pub unsafe extern "cdecl" fn test(argc: byondapi_sys::u4c, argv: *const ByondValue) -> ByondValue {
    if argc == 1 {
        if !argv.is_null() {
            let mut value = argv.read();
            value.set_number(10.0);
            (1.0).into()
        } else {
            (-2.0).into()
        }
    } else {
        (-4.0).into()
    }
}
