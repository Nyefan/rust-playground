use std::collections::HashMap;

pub fn test() {
    demo_vec();
    demo_string();
    demo_hash_map();
}

fn demo_vec() {
    demo_vec_access();
    demo_vec_iteration();
    demo_vec_enum();
}

fn demo_vec_access() {
    let v = vec![1, 2, 3, 4, 5];

    let _first = v.get(0);
    let _third: &i32 = &v[2];

    let v_index = 2;

    match v.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); }
        None => { println!("Unreachable element at index: {}", v_index); }
    }

    // Causes panic!
    // let does_not_exist = &v[100];

    // Returns None
    // let does_not_exist = v.get(100);

    // Doesn't compile due to mutable borrows of v.get(0) and &v[2] above
    // v.push(6);
}

fn demo_vec_iteration() {
    let mut v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
}

fn demo_vec_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn demo_string() {
    demo_string_create();
    demo_string_mutate();
    demo_string_concatenate();
    demo_string_index();
}

fn demo_string_create() {
    let _s = String::new();
    let data = "initial contents";
    let _s2 = data.to_string();
    let _s3 = "initial contents".to_string();
    let _s4 = String::from("initial contents");
}

fn demo_string_mutate() {
    let mut sfoo = String::from("foo");
    sfoo.push_str("bar");
    println!("sfoo is {}", sfoo);

    let mut sfoo2 = String::from("foo");
    let sbar = "bar";
    sfoo2.push_str(sbar);
    println!("sfoo2 is {}", sfoo2);

    let mut slol = String::from("lo");
    slol.push('l'); //push takes a single character
    println!("slol is {}", slol);
}

fn demo_string_concatenate() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //Note s1 has been moved here and can no longer be used
    println!("s2 is {} and s3 is {}", s2, s3);

    let s4 = s3.clone() + "  Is anyone there?"; //cloning the string creates a new copy, which means the original is still available
    println!("s4 is {}", s4);

    let s5 = format!("{}{}", s3, String::from("  Is anyone there?")); //alternately, we can use the format! macro, which doesn't take ownership of anything
    println!("s5 is {}", s5);
}

fn demo_string_index() {
    let _s1 = String::from("hello");
    // This fails because Strings are wrappers around Vec<u8>, meaning a naive indexer would potentially return half of a UTF-8 codepoint
//    let h = s1[0];

    let _s2 = String::from("नमस्ते");
    // s2 as bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // s2 as Unicode values
    // ['न', 'म', 'स', '्', 'त', 'े']
    // s2 as graphemes
    // ["न", "म", "स्", "ते"]

    let s3 = String::from("Здравствуйте");

    let s4 = &s3[0..4];
    println!("s4 is {}", s4);

//    This would cause a panic!
//    `thread 'main' panicked at 'byte index 1 is not a char boundary`
//    let s5 = &s3[0..1];

    let mut chars = String::new();
    chars.push('[');
    for c in s3.chars() {
        chars.push(c);
        chars.push_str(", ");
    }
    chars.push(']');
    println!("chars is {}", chars);

    let mut bytes = String::new();
    bytes.push('[');
    for b in s3.bytes() {
        bytes.push_str(&format!("{}, ", b));
    }
    bytes.push(']');
    println!("bytes is {}", bytes);
}


fn demo_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

#[cfg(test)]
mod test;