#![allow(clippy::missing_safety_doc)]

use byondapi::{
    list::ByondValueList,
    map::{byond_block, byond_length, ByondXYZ},
    parse_args,
    value::{pointer::ByondValuePointer, ByondValue},
};

#[allow(dead_code)]
fn write_log<T: AsRef<[u8]>>(x: T) {
    std::fs::write("./rust_log.txt", x).unwrap()
}

use std::panic;
fn setup_panic_handler() {
    panic::set_hook(Box::new(|info| {
        write_log(format!("Panic {:#?}", info));
    }))
}

#[no_mangle]
pub unsafe extern "C" fn test_connection(
    _argc: byondapi_sys::u4c,
    _argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    ByondValue::new_num(69.0)
}

#[no_mangle]
pub unsafe extern "C" fn test_args(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);
    assert_eq!(args.len(), 1);
    args[0].clone()
}

#[no_mangle]
pub unsafe extern "C" fn send_test(_argc: byondapi_sys::u4c, _argv: *mut ByondValue) -> ByondValue {
    setup_panic_handler();
    // let args = parse_args(argc, argv);
    let new_value = ByondValue::new_str("Meow").unwrap();

    std::thread::spawn(move || {
        std::mem::drop(new_value);
    });

    ByondValue::null()
}

#[no_mangle]
pub unsafe extern "C" fn test_ptr(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);
    let pointer = match ByondValuePointer::new(args[0].clone()) {
        Ok(ptr) => ptr,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let strobj = match pointer.read() {
        Ok(ptr) => ptr,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let new_name: ByondValue = format!("awa{}", strobj.get_string().unwrap())
        .try_into()
        .unwrap();

    match pointer.write(&new_name) {
        Ok(_) => {}
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    ByondValue::null()
}

#[no_mangle]
pub unsafe extern "C" fn test_proc_call(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    // FIXME: Byond will change this in the future
    let result = args[0].call("get name", &[]);

    match result {
        Ok(res) => res,
        Err(e) => format!("{:#?}", e).try_into().unwrap(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_readwrite_var(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);
    let object = &args[0];

    match object.read_var("name") {
        Ok(res) => res,
        Err(e) => format!("{:#?}", e).try_into().unwrap(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_list_push(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let mut list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    match list.push(&ByondValue::new_num(8.0)) {
        Ok(_) => {}
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    list.try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_double(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let collection: Vec<ByondValue> = list
        .iter()
        .map(|f| (f.get_number().unwrap() * 2.).try_into().unwrap())
        .collect();

    let list: ByondValueList = collection.as_slice().try_into().unwrap();

    list.try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_index(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    list[3].clone()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_pop(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let mut list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let element = match list.pop() {
        Ok(x) => x,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    if list.0.count != 4 {
        return "pop did not actually remove item from list"
            .try_into()
            .unwrap();
    }

    element
}

#[no_mangle]
pub unsafe extern "C" fn test_length_with_list(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let list: ByondValueList = match (&args[0]).try_into() {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    let value: ByondValue = match list.try_into() {
        Ok(x) => x,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    match byond_length(&value) {
        Ok(x) => x,
        Err(e) => format!("{:#?}", e).try_into().unwrap(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_block(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    setup_panic_handler();
    let _args = parse_args(argc, argv);

    let block = match byond_block(
        ByondXYZ::with_coords((1, 1, 1)),
        ByondXYZ::with_coords((2, 2, 1)),
    ) {
        Ok(list) => list,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    if block.len() != 4 {
        return format!("block returned {} turfs when we expected 4", block.len())
            .try_into()
            .unwrap();
    }

    (block.len() as f32).into()
}

#[no_mangle]
pub unsafe extern "C" fn test_length_with_str(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    match byond_length(&args[0]) {
        Ok(x) => x,
        Err(e) => format!("{:#?}", e).try_into().unwrap(),
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_list_key_lookup(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let list = &args[0];

    let num: f32 = match list.read_list_index(&ByondValue::try_from("cat").unwrap()) {
        Ok(x) => x.try_into().unwrap(),
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };
    assert_eq!(num, 7.0);

    let num: f32 = match list.read_list_index(&ByondValue::try_from("dog").unwrap()) {
        Ok(x) => x.try_into().unwrap(),
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };
    assert_eq!(num, 5.0);

    let num: f32 = match list.read_list_index(&ByondValue::try_from("parrot").unwrap()) {
        Ok(x) => x.try_into().unwrap(),
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };
    assert_eq!(num, 4.0);

    if let Err(e) = list.write_list_index(
        &ByondValue::try_from("parrot").unwrap(),
        &ByondValue::try_from(14.0).unwrap(),
    ) {
        return format!("{:#?}", e).try_into().unwrap();
    };

    ByondValue::new()
}
