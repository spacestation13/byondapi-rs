#![allow(clippy::missing_safety_doc)]

use byondapi::{byond_string, map::*, prelude::*};

#[test]
fn generate_binds() {
    byondapi::generate_bindings(env!("CARGO_CRATE_NAME"));
}

fn write_log<T: AsRef<[u8]>>(x: T) {
    std::fs::write("./rust_log.txt", x).unwrap()
}

fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|info| {
        write_log(format!("Panic {:#?}", info));
    }))
}

#[byondapi::bind]
fn test_connection() {
    setup_panic_handler();
    Ok(ByondValue::new_num(69.0))
}

#[byondapi::bind_raw_args]
fn test_args() {
    setup_panic_handler();
    assert_eq!(args.len(), 2);
    Ok(args[1])
}

#[byondapi::bind]
fn test_ptr(ptr: ByondValue) {
    setup_panic_handler();
    let pointer = ByondValuePointer::new(ptr)?;

    let strobj = pointer.read()?;

    let new_name: ByondValue = format!("awa{}", strobj.get_string()?).try_into()?;

    pointer.write(&new_name)?;
    Ok(ByondValue::null())
}

#[byondapi::bind]
fn test_proc_call(object: ByondValue) {
    Ok(object.call("get_name", &[])?)
}

#[byondapi::bind]
fn test_readwrite_var(object: ByondValue) {
    setup_panic_handler();

    object.read_var_id(byond_string!("name"))?.get_string()?;

    Ok(object.read_string("name")?.try_into()?)
}
#[byondapi::bind]
fn test_list_push(mut list: ByondValue) {
    setup_panic_handler();

    list.push_list(ByondValue::new_num(8.0))?;

    Ok(list)
}

#[byondapi::bind]
fn test_list_double(list: ByondValue) {
    setup_panic_handler();

    let collection = list
        .iter()?
        .map(|(v, _)| (v.get_number().unwrap() * 2.).into())
        .collect::<Vec<ByondValue>>();

    Ok(collection.as_slice().try_into()?)
}

#[byondapi::bind]
fn test_list_index(list: ByondValue) {
    setup_panic_handler();

    Ok(list.read_list_index(3.0)?)
}

#[byondapi::bind]
fn test_list_pop(mut list: ByondValue) {
    setup_panic_handler();

    let element = list.pop_list()?;

    if list.builtin_length()?.get_number()? as u32 != 4 {
        return Err(byondapi::Error::BindError(format!(
            "pop did not actually remove item from list"
        )));
    }

    Ok(element.unwrap())
}

#[byondapi::bind]
fn test_length_with_list(list: ByondValue) {
    setup_panic_handler();
    Ok(list.builtin_length()?)
}

#[byondapi::bind]
fn test_block() {
    setup_panic_handler();

    let block = byond_block(
        ByondXYZ::with_coords((1, 1, 1)),
        ByondXYZ::with_coords((2, 2, 1)),
    )?;

    if block.len() != 4 {
        return Err(byondapi::Error::BindError(format!(
            "block returned {} turfs when we expected 4",
            block.len()
        )));
    }

    Ok((block.len() as f32).into())
}

#[byondapi::bind]
fn test_length_with_str(object: ByondValue) {
    setup_panic_handler();

    Ok(byond_length(&object)?)
}
#[byondapi::bind]
fn test_list_key_lookup(mut list: ByondValue) {
    setup_panic_handler();

    let num: f32 = list.read_list_index("cat")?.try_into()?;
    assert_eq!(num, 7.0);

    let num: f32 = list.read_list_index("dog")?.try_into()?;
    assert_eq!(num, 5.0);

    let num: f32 = list.read_list_index("parrot")?.try_into()?;
    assert_eq!(num, 4.0);

    list.write_list_index("parrot", 14.0)?;

    let key: String = list.read_list_index(3.0)?.try_into()?;

    assert_eq!("parrot", key);

    let map = list
        .iter()?
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

    Ok(Default::default())
}

#[byondapi::bind]
fn test_ref(turf: ByondValue) {
    setup_panic_handler();

    let turf_type = turf.get_type();
    let turf_id = turf.get_ref()?;

    Ok(ByondValue::try_from(format!(
        "turf_id: {turf_id}, turf_type: {turf_type}"
    ))?)
}

#[byondapi::bind]
fn test_non_assoc_list(list: ByondValue) {
    setup_panic_handler();

    let map = list
        .iter()?
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

    Ok(Default::default())
}

#[byondapi::bind]
fn test_list_read(list: ByondValue) {
    setup_panic_handler();

    let map = list.get_list_keys()?;
    let values = map
        .into_iter()
        .map(|item| item.get_string().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        values,
        vec!["cat".to_owned(), "dog".to_owned(), "parrot".to_owned()]
    );

    Ok(Default::default())
}
