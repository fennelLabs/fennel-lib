use crate::whiteflag::wf_codec::*;

fn assert_array_eq<T: PartialEq + std::fmt::Debug>(l: Vec<T>, r: Vec<T>) {
    let success = l.iter().eq(r.iter());
    if !success {
        println!("expected: {:?}\nwas: {:?}", l, r);
    }
    assert!(success);
}

#[test]
fn test_shift_right_0() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let shifted_bytes = common::shift_right(original.clone(), 0);

    assert_array_eq(original, shifted_bytes);
}

#[test]
fn test_shift_right_1() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let expected: Vec<u8> = vec![0x0A, 0x6F, 0xA0];

    assert_array_eq(expected.clone(), common::shift_right(original.clone(), 3));
    assert_array_eq(expected.clone(), common::shift_left(original.clone(), -3));
}

#[test]
fn test_shift_right_2() {
    let original: Vec<u8> = vec![0xF6, 0x38, 0x6D];
    let expected: Vec<u8> = vec![0x07, 0xB1, 0xC3, 0x68];
    let shifted_bytes = common::shift_right(original.clone(), 5);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_shift_right_3() {
    let original: Vec<u8> = vec![0xE6, 0x38, 0x6D, 0x84];
    let expected: Vec<u8> = vec![0x0E, 0x63, 0x86, 0xD8, 0x40];

    assert_array_eq(expected.clone(), common::shift_right(original.clone(), 12));
    assert_array_eq(expected.clone(), common::shift_left(original.clone(), -12));
}

#[test]
fn test_shift_left_0() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let shifted_bytes = common::shift_left(original.clone(), 0);

    assert_array_eq(original, shifted_bytes);
}

#[test]
fn test_shift_left_1() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let expected: Vec<u8> = vec![0x9B, 0xE8];

    assert_array_eq(expected.clone(), common::shift_left(original.clone(), 3));
    assert_array_eq(expected.clone(), common::shift_right(original.clone(), -11));
}

#[test]
fn test_shift_left_2() {
    let original: Vec<u8> = vec![0xE6, 0x38, 0x87];
    let expected: Vec<u8> = vec![0x1C, 0x43, 0x80];
    let shifted_bytes = common::shift_left(original.clone(), 7);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_shift_left_3() {
    let original: Vec<u8> = vec![0xD4, 0x4B, 0x93, 0x93];
    let expected: Vec<u8> = vec![0x89, 0x72, 0x72, 0x60];
    let shifted_bytes = common::shift_left(original.clone(), 5);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_shift_left_4() {
    let original: Vec<u8> = vec![0xE6, 0x38, 0x87, 0x0f];
    let expected: Vec<u8> = vec![0x63, 0x88, 0x70, 0xf0];
    let shifted_bytes = common::shift_left(original.clone(), 4);

    assert_array_eq(expected, shifted_bytes);
}
