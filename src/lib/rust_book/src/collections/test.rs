use std::panic;
use std::collections::HashMap;

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

#[test]
fn vec_iteration() {
    let mut vec = vec![100, 32, 57];
    let expect_borrow = vec![100, 32, 57];
    let expect_mutate = vec![150, 82, 107];

    for (index, value) in vec.iter().enumerate() {
        assert_eq!(Some(value), expect_borrow.get(index));
    }

    for value in &mut vec {
        *value += 50;
    }

    for (index, value) in vec.iter().enumerate() {
        assert_eq!(Some(value), expect_mutate.get(index));
    }
}

/// ```compile_fail
/// let row = vec![3, String::from("blue"), 10.12];
/// ```
/// ```compile_fail
/// let row = vec![3, 4, 5];
/// ```
/// ```
/// let row = vec![3, String::from("blue"), 10.12];
/// ```
#[test]
fn vec_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    assert!(panic::catch_unwind(|| {
        let _row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }).is_ok());
}

#[test]
fn string_create() {
    assert!(panic::catch_unwind(|| {
        let _s = String::new();
        let data = "initial contents";
        let _s2 = data.to_string();
        let _s3 = "initial contents".to_string();
        let _s4 = String::from("initial contents");
    }).is_ok());
}

#[test]
fn string_mutate() {
    let mut sfoo = String::from("foo");
    sfoo.push_str("bar");
    assert_eq!(String::from("foobar"), sfoo);

    let mut sfoo2 = String::from("foo");
    let sbar = "bar";
    sfoo2.push_str(sbar);
    assert_eq!(String::from("foobar"), sfoo2);
    //String::push_str returns ownership after making a copy
    assert!(panic::catch_unwind(|| {
        println!("{}", sbar);
    }).is_ok());

    let mut slol = String::from("lo");
    slol.push('l');
    assert_eq!(String::from("lol"), slol);
}

/// This fails because Strings are wrappers around Vec<u8>, meaning
/// a naive indexer would potentially return half of a UTF-8 codepoint
/// ```compile_fail
/// let _s1 = String::from("hello");
/// let _h = s1[0]
/// ```
#[test]
fn string_index() {
    let _s2 = String::from("नमस्ते");
    // s2 as bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // s2 as Unicode values
    // ['न', 'म', 'स', '्', 'त', 'े']
    // s2 as graphemes
    // ["न", "म", "स्", "ते"]

    let s3 = String::from("Здравствуйте");
    let s4 = &s3[0..4];
    assert_eq!("Зд", s4);

    // `thread 'main' panicked at 'byte index 1 is not a char boundary'`
    assert!(panic::catch_unwind(|| {
        let _s5 = &s3[0..1];
    }).is_err());

    let mut chars = String::new();
    chars.push('[');
    for c in s3.chars() {
        chars.push(c);
        chars.push_str(", ");
    }
    chars.push(']');
    assert_eq!("[З, д, р, а, в, с, т, в, у, й, т, е, ]", chars);

    let mut bytes = String::new();
    bytes.push('[');
    for b in s3.bytes() {
        bytes.push_str(&format!("{}, ", b));
    }
    bytes.push(']');
    assert_eq!("[208, 151, 208, 180, 209, 128, 208, 176, 208, 178, 209, 129, 209, 130, 208, 178, 209, 131, 208, 185, 209, 130, 208, 181, ]", bytes);
}

#[test]
fn hash_map_create() {
    let blue = || String::from("Blue");
    let yellow = || String::from("Yellow");
    let red = || String::from("Red");
    let mut scores = HashMap::new();
    scores.insert(blue(), 10);
    scores.insert(yellow(), 50);
    assert_eq!(Some(&10), scores.get(&blue()));
    assert_eq!(None, scores.get(&red()));


    let teams = vec![blue(), yellow()];
    let initial_scores = vec![10, 50];
    let zip_scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    assert_eq!(scores, zip_scores);

    scores.insert(blue(), 25);
    assert_eq!(Some(&25), scores.get(&blue()));

    scores.entry(yellow()).or_insert(55);
    assert_eq!(Some(&50), scores.get(&yellow()));
    scores.entry(red()).or_insert(80);
    assert_eq!(Some(&80), scores.get(&red()));
}

#[test]
fn hash_map_update() {
    let text = "hello world, wonderful world";
    let mut map = HashMap::new();

    text.split_whitespace()
        .map(|w: &str| w.trim_matches(|c: char| c.is_ascii_punctuation()))
        .for_each(|w: &str| {
            let count = map.entry(w).or_insert(0);
            *count += 1;
        });

    let expected_map: HashMap<_, _> = vec!["hello", "wonderful", "world"].into_iter().zip(vec![1, 1, 2].into_iter()).collect();
    assert_eq!(expected_map, map);
}