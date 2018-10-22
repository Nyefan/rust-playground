pub fn test() {
    println!("Hello, world!");
    let s: String = "hello, world".to_string();
    println!("{}", first_space_index(&s));
    println!("{}", first_word_slice("slice first word"));
    println!("{}", second_word_slice("slice second word"));
    println!("{}", second_word_slice_2("shouldbeempty"));
    println!("{}", second_word_slice_2("second \"word\""));
    println!("{}", second_word_slice_2("the middle word"));
}

fn first_space_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut first_space = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_space == 0 {
                first_space = i;
            } else {
                return &s[first_space + 1..i];
            }
        }
    }
    &s[first_space + 1..]
}

//TODO: learn to attach debugger in rust
fn second_word_slice_2(s: &str) -> &str {
//    println!("Receiving {}", &s);
    let bytes = s.as_bytes();

    let mut first_space = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
//            println!("Setting first_space to {}", i);
            first_space = i;
            break;
        }
    }

    if first_space == 0 {
//        println!("Returning &s[0..0]: {}", &s[0..0]);
        return &s[0..0]; //should be an error result? dunno how yet
    }

    for (i, &item) in bytes[first_space + 1..].iter().enumerate() {
        if item == b' ' {
//            println!("Returning &s[{}..{}]: {}", first_space + 1, first_space + 1 + i, &s[first_space + 1..first_space + 1 + i]);
            return &s[first_space + 1..first_space + 1 + i];
        }
    }

//    println!("Returning &s[{}..]: {}", first_space + 1, &s[first_space + 1..]);
    &s[first_space + 1..]
}
