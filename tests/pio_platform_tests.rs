#[test]
fn bool_to_bit_true() {
    assert_eq!(1, piolib_rs::bool_to_bit!(true));
}
#[test]
fn bool_to_bit_false() {
    assert_eq!(0, piolib_rs::bool_to_bit!(false));
}
#[test]
fn count_of_array() {
    assert_eq!(3, piolib_rs::count_of!([1, 2, 3]));
}
#[test]
fn count_of_array_ref() {
    let x = [1, 2, 3, 4];
    let y = &x;
    assert_eq!(4, piolib_rs::count_of!(y));
}
