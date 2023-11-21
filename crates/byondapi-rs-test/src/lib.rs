#![allow(clippy::missing_safety_doc)]

use byondapi::{
    map::{byond_block, byond_length, ByondXYZ},
    parse_args,
    typecheck_trait::ByondTypeCheck,
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
    args[0]
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

    match object.read_string("name") {
        Ok(res) => res.try_into().unwrap(),
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

    let mut list = args[0];

    match list.push_list(ByondValue::new_num(8.0)) {
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

    let list = args[0];

    let collection: Vec<ByondValue> = list
        .iter()
        .unwrap()
        .map(|(v, _)| (v.get_number().unwrap() * 2.).try_into().unwrap())
        .collect();

    collection.as_slice().try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_index(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let list = args[0];

    list.read_list_index(3.0).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_pop(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let mut list = args[0];

    let element = match list.pop_list() {
        Ok(x) => x,
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };

    if list.builtin_length().unwrap().get_number().unwrap() as u32 != 4 {
        return "pop did not actually remove item from list"
            .try_into()
            .unwrap();
    }

    element.unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_length_with_list(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let list = args[0];

    match list.builtin_length() {
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

    let list = args.get_mut(0).unwrap();

    let num: f32 = match list.read_list_index("cat") {
        Ok(x) => x.try_into().unwrap(),
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };
    assert_eq!(num, 7.0);

    let num: f32 = match list.read_list_index("dog") {
        Ok(x) => x.try_into().unwrap(),
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };
    assert_eq!(num, 5.0);

    let num: f32 = match list.read_list_index("parrot") {
        Ok(x) => x.try_into().unwrap(),
        Err(e) => return format!("{:#?}", e).try_into().unwrap(),
    };
    assert_eq!(num, 4.0);

    if let Err(e) = list.write_list_index("parrot", 14.0) {
        return format!("{:#?}", e).try_into().unwrap();
    };

    let key: String = list.read_list_index(3.0).unwrap().try_into().unwrap();

    assert_eq!("parrot", key);

    let map = list
        .iter()
        .unwrap()
        .map(|(k, v)| (k.get_string().unwrap(), v.get_number().unwrap() as u32))
        .collect::<Vec<_>>();

    assert_eq!(
        map,
        vec![
            ("cat".to_owned(), 7),
            ("dog".to_owned(), 5),
            ("parrot".to_owned(), 14)
        ]
    );

    ByondValue::new()
}

#[no_mangle]
pub unsafe extern "C" fn test_ref(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);

    let turf = args.get(0).unwrap();
    let turf_type = turf.get_type();
    let turf_id = turf.get_ref().unwrap();

    ByondValue::try_from(format!("turf_id: {turf_id}, turf_type: {turf_type}")).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_non_assoc_list(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);
    let list = args.get(0).unwrap();

    let map = list
        .iter()
        .unwrap()
        .map(|(k, v)| {
            if !v.is_null() {
                panic!("value is not null")
            }
            k.get_string().unwrap()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        map,
        vec!["cat".to_owned(), "dog".to_owned(), "parrot".to_owned()]
    );

    ByondValue::new()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_read(
    argc: byondapi_sys::u4c,
    argv: *mut ByondValue,
) -> ByondValue {
    setup_panic_handler();
    let args = parse_args(argc, argv);
    let list = args.get(0).unwrap();

    let map = list.get_list().unwrap();
    let (num, string): (Vec<_>, Vec<_>) =
        map.into_iter()
            .partition(|value| if value.is_num() { true } else { false });
    let num = num
        .into_iter()
        .map(|value| value.get_number().unwrap() as usize)
        .collect::<Vec<_>>();
    let string = string
        .into_iter()
        .map(|value| value.get_string().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(num, vec![0, 1, 5]);

    assert_eq!(
        string,
        vec!["cat".to_owned(), "dog".to_owned(), "parrot".to_owned()]
    );

    ByondValue::new()
}
