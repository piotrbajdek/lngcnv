// LNGCNV VERSION 1.10.1 / MIT LICENSE / COPYRIGHT © 2022–2024 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod lang;
pub mod menu;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let blue_underlined = "\x1b[34;4m";
    let red = "\x1b[31m";
    let cyan = "\x1b[36m";
    let yellow = "\x1b[93m";
    let grey = "\x1b[38;5;240m";

    menu::documentation(reset, blue_underlined, cyan, yellow, grey);
    lang::list(reset, red, cyan, yellow);
}
