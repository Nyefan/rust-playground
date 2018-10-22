use std::panic;

use super::*;
use std::option::Option::None;

#[test]
fn demo() {
    demo_vec();
    demo_string();
    demo_hash_map();
}

#[test]
fn vec_access() {
    let v = vec![1, 2, 3, 4, 5];

    let first = v.get(0);
    assert_eq!(first.expect("Expected 1"), &1);

    let third: &i32 = &v[2];
    assert_eq!(third, &3);

    let v_index = 2;
    let v_index_value = match v.get(v_index) {
        Some(i) => { i }
        None => { panic!(); }
    };
    assert_eq!(v_index_value, &3);

    // Accessing data through &v[n] where n is out of bounds causes a panic
    assert!(panic::catch_unwind(|| { let _ = &v[100]; }).is_err());

    // Accessing data through v.get(n) where n is out of bounds returns None
    assert_eq!(v.get(100), None);

    // Doesn't compile because v cannot be borrowed mutably
    // v.push(6);
}

