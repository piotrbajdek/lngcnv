// LNGCNV VERSION 1.6.0-BETA.5 / MIT LICENSE © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod lang;
pub mod menu;

// MAIN

fn main() {
    menu::documentation();
    lang::list();
}
