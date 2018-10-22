pub fn test() {
    ex_mutable_string();
    ex_object_reference();
    ex_ownership_on_heap_data();
}

fn ex_mutable_string() {
    let mut s = String::from("hello");
    println!("{}", s);
    s.push_str(", world!");
    println!("{}", s);
}

fn ex_object_reference() {
    let mut a = 5;
    let b = a;
    a = 6;
    println!("a: {}, b: {}", a, b);

    let s1 = String::from("hello");
    let mut s2 = s1; //String is not 'Copy'; therefore this is a "move"
    s2.push_str(", world!");
    println!("s2: {}", s2);
}

fn ex_ownership_on_heap_data() {
    demo_losing_ownership();
    demo_take_and_return();
    demo_take_and_return_tuple();
    demo_take_reference();
    demo_mut_reference();
    demo_copy_trait();
}

fn demo_losing_ownership() {
    let s = String::from("goodbye");

    let fs: fn(String) = |s: String| {
        println!("{}, world!", s);
        // s.drop() is called here
    };

    fs(s);

//    s is no longer accessible, and this line results in a compiler error
//    println!("{}, world!", s);
}

fn demo_take_and_return() {
    let freturn: fn(String) -> String = |s: String| {
        println!("{}, world!", s);
        s
    };

    let sreturn = String::from("Goodbye");
    let sreturn = freturn(sreturn);
    println!("{} and hello again!", sreturn);
}

fn demo_take_and_return_tuple() {
    let s1 = String::from("testString");

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }

    let (s2, length) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, length);
}

fn demo_take_reference() {
    let s1 = String::from("referenceString");

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let length = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, length);
}

fn demo_mut_reference() {
    let mut s1 = String::from("mutable");

    fn alter_string(s: &mut String) {
        s.push_str("String");
    }

    alter_string(&mut s1);

    println!("{}", s1);
}

fn demo_copy_trait() {
    let x: i32 = 50;

    (|x: i32| {
        println!("x: {}", x);
        // x is 'Copy', so only the local value is dropped
    }).call((x, ));

    println!("x: {}", x);
}