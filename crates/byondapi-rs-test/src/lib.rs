use byondapi::{typecheck_trait::ByondTypeCheck, value::ByondValue};

#[no_mangle]
/// # Safety
/// It's fucked
pub unsafe extern "cdecl" fn test(argc: byondapi_sys::u4c, argv: *const ByondValue) -> ByondValue {
    let value: ByondValue = argv.read();

    let new_num: ByondValue = (1.0).into();
    let res = new_num.write_ptr(&value);

    let _ = std::fs::write(
        "meow.txt",
        format!("argv: {:#?}\n result: {:#?}", value, res),
    );

    if res.is_ok() {
        (1.0).into()
    } else {
        (-1.0).into()
    }
}
