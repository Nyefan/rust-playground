pub fn test() {
    println!("Hello, world!");
    demo_ip_addr_kind();
    demo_ip_addr();
    demo_message();
    demo_option();
    demo_match();
    demo_if_let();
}


fn demo_ip_addr_kind() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }


    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}


fn demo_ip_addr() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}

fn demo_message() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quitting!"),
                Message::Move { x, y } => println!("Moving to ({}, {})!", x, y),
                Message::Write(s) => { println!("{}", s); }
                Message::ChangeColor(r, g, b) => {
                    println!("Changing color to ({}, {}, {})", r, g, b);
                }
//                _ => println!("Shouldn't reach here!"),
            };
        }
    }

    Message::Quit.call();
    Message::Move { x: 8, y: 0 }.call();
    Message::Write(String::from("String to print")).call();
    Message::ChangeColor(1, 2, 3).call();
}

fn demo_option() {
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
}

fn demo_match() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        Dollar,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            Coin::Dollar => 100,
        }
    }

    println!("Value of a Penny: {}", value_in_cents(Coin::Penny));
    println!("Value of a Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Value of a Dime: {}", value_in_cents(Coin::Dime));
    println!("Value of a Quarter: {}", value_in_cents(Coin::Quarter));
    println!("Value of a Dollar: {}", value_in_cents(Coin::Dollar));
}

fn demo_if_let() {
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    #[derive(Debug)]
    enum State {
        Alabama,
        Alaska,
        Arizona,
        Arkansas,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State),
        Dollar,
    }

    fn count_non_quarters(coins: &Vec<Coin>) -> u32 {
        let mut count = 0;
        for coin in coins.iter() {
            if let Coin::Quarter(state) = coin {
                println!("State quarter from {:?}!", state);
            } else {
                count += 1;
            }
        }
        count
    }

    let coins = vec![Coin::Penny, Coin::Quarter(State::Alabama), Coin::Penny, Coin::Dime,
                     Coin::Nickel, Coin::Dollar, Coin::Quarter(State::Alaska),
                     Coin::Quarter(State::Arizona), Coin::Quarter(State::Arkansas)];
    println!("There are {} non-quarters", count_non_quarters(&coins));
}