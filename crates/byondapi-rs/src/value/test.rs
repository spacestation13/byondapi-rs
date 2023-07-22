use super::*;

#[test]
fn init_and_drop_bare() {
    let meow = ByondValue::new();
    std::hint::black_box(&meow);
    std::mem::drop(meow);
}

#[test]
fn init_and_drop_string() {
    let meow = ByondValue::new_str("Meow meow meow meorw lwemow");
    std::hint::black_box(&meow);
    std::mem::drop(meow);
}

#[test]
fn conversions() {
    let null = ByondValue::new();
    assert!(null.is_null());
    assert!(!null.is_num());
    assert!(!null.is_str());
    assert!(!null.is_list());

    let number: ByondValue = (42.0).into();
    assert!(!number.is_null());
    assert!(number.is_num());
    assert!(!number.is_str());
    assert!(!number.is_list());

    let string: ByondValue = "meow".try_into().unwrap();
    assert!(!string.is_null());
    assert!(!string.is_num());
    assert!(string.is_str());
    assert!(!string.is_list());
}

#[test]
fn equality() {
    let number: ByondValue = (42.0).into();
    let number2: ByondValue = (42.0).into();
    assert!(number.eq(&number2));

    let nan: ByondValue = (f32::NAN).into();
    let nan2: ByondValue = (f32::NAN).into();
    assert!(nan.ne(&nan2));

    let string: ByondValue = "meow".try_into().unwrap();
    let string2: ByondValue = "meow".try_into().unwrap();
    assert!(string.eq(&string2));

    let null: ByondValue = ByondValue::new();
    let null2: ByondValue = ByondValue::new();
    assert!(null.eq(&null2));
}
