use std::vec::Vec;

pub fn test() {
    ex_mut();
    println!("{}", ex_tuple().0);
    println!("{}", ex_array()[0]);
    println!("{}", ex_vector()[0]);
    ex_if(3);
    ex_if(6);
    println!("{}", ex_loop());
    ex_while();
    ex_for();
    ex_for_slice();
}

fn ex_mut() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn ex_tuple() -> (i32, f64, u8) {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    tuple
}

fn ex_array() -> [i32; 5] {
    let array = [1, 2, 3, 4, 5];
    array
}

fn ex_vector() -> Vec<i32> {
    let vector = vec![1, 2, 3];
    vector
}

fn ex_if(number: i32) {
    let number = if number < 5 {
        true
    } else {
        false
    };

    if number {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn ex_loop() -> i32 {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 3 {
            break counter + 1;
        }
    }
}

fn ex_while() {
    let mut counter = 0;
    while counter < 4 {
        println!("while: {}", counter);
        counter += 1;
    }
}

fn ex_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("a.iter(): {}", element);
    }
}

fn ex_for_slice() {
    //note that slices are [i, j) on (i..j)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
