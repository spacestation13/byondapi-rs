#![allow(clippy::missing_safety_doc)]
use byondapi::value::ByondValue;

fn parse_args(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> &'static mut [ByondValue] {
    unsafe { std::slice::from_raw_parts_mut(argv, argc as usize) }
}

#[no_mangle]
pub unsafe extern "C" fn test_connection(
    _argc: byondapi_sys::u4c,
    _argv: *mut ByondValue,
) -> ByondValue {
    ByondValue::new_num(69.0)
}

#[no_mangle]
pub unsafe extern "C" fn test_args(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    let args = parse_args(argc, argv);
    assert_eq!(args.len(), 1);
    args[0].clone()
}
