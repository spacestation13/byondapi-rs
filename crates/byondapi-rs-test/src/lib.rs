use byondapi::value::ByondValue;

#[no_mangle]
/// # Safety
/// It's fucked
pub unsafe extern "cdecl" fn test_obj(
    argc: byondapi_sys::u4c,
    argv: *const ByondValue,
) -> ByondValue {
    if argc != 1 {
        return ByondValue::null();
    }

    let mut value: ByondValue = argv.read();
    let _ = value.write_var("name", &"woof".try_into().unwrap());
    let ret = value.read_var("name").unwrap();

    value.call("testproc", &[]).unwrap();

    ret
}

#[no_mangle]
/// # Safety
/// It's fucked
pub unsafe extern "cdecl" fn test_ptr(
    argc: byondapi_sys::u4c,
    argv: *const ByondValue,
) -> ByondValue {
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
