#![feature(fn_traits)]

extern crate rust_book;

use rust_book::*;

fn main() {
    println!("Test Collections");
    collections::test();
    println!("Test Enums");
    enums::test();
    println!("Test Hello Cargo");
    hello_cargo::test();
    println!("Test Ownership");
    ownership::test();
    println!("Test Slices");
    slices::test();
    println!("Test Structs");
    structs::test();
    println!("Test Variables");
    variables::test();
}