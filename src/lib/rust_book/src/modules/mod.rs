pub mod client;
pub mod network;

pub mod outermost {
    pub fn middle_function() {
        middle_secret_function();
    }

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            super::super::outermost::middle_secret_function();
            inner_secret_function();
        }

        fn inner_secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
//    outermost::middle_secret_function();
//    outermost::inside::inner_function();
//    outermost::inside::inner_secret_function();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::a::series::of::nested_modules;
    use super::TrafficLight::{Red, Yellow};
    use super::outermost;

    #[test]
    fn main() {
        nested_modules();
        let _red = Red;
        let _yellow = Yellow;
        let _green = super::TrafficLight::Green;
        outermost::middle_function();
    }
}
