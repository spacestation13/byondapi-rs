#![allow(clippy::missing_safety_doc)]

use byondapi::value::ByondValue;

fn parse_args(argc: byondapi_sys::u4c, argv: *const ByondValue) -> &'static [ByondValue] {
    unsafe { std::slice::from_raw_parts(argv, argc as usize) }
}

#[no_mangle]
pub unsafe extern "C" fn test_connection(
    _argc: byondapi_sys::u4c,
    _argv: *const ByondValue,
) -> ByondValue {
    ByondValue::new_num(69.0)
}

#[no_mangle]
pub unsafe extern "C" fn test_args(argc: byondapi_sys::u4c, argv: *const ByondValue) -> ByondValue {
    let args = parse_args(argc, argv);
    assert_eq!(args.len(), 1);
    args[0].clone()
}

#[no_mangle]
pub unsafe extern "C" fn send_test(argc: byondapi_sys::u4c, argv: *const ByondValue) -> ByondValue {
    // let args = parse_args(argc, argv);
    let new_value = ByondValue::new_str("Meow").unwrap();

    std::thread::spawn(move || {
        std::mem::drop(new_value);
    });

    ByondValue::null()
}
