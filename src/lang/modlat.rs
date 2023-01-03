// LNGCNV VERSION 1.8.0 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE MODLAT

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)]

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// LATIN: IPA

pub fn ipalat(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let dot = original_text.to_owned() + ".";
    let lowercase = &dot.to_lowercase();
    let result = &lowercase
        .replace('ꟾ', "ī")
        .replace('g', "ɡ")
        .replace('v', "u")
        .replace("uu", "vu")
        .replace("ui", "vi")
        .replace("ue", "ve")
        .replace("ua", "va")
        .replace("uo", "vo")
        .replace("uū", "vū")
        .replace("uī", "vī")
        .replace("uē", "vē")
        .replace("uā", "vā")
        .replace("uō", "vō")
        .replace("bs", "ps")
        .replace("bt", "pt")
        .replace('s', "s̺")
        .replace("s̺b", "s̬b")
        .replace("s̺d", "s̬d")
        .replace("s̺ɡ", "s̬ɡ")
        .replace("s̺m", "s̬m")
        .replace("s̺n", "s̬n")
        .replace("nd", "md")
        .replace("n d", "m d")
        .replace("np", "mp")
        .replace("n p", "m p")
        .replace("inf", "ĩːf")
        .replace("nf", "ɱf")
        .replace("n f", "ɱ f")
        .replace("nm", "mm")
        .replace("n m", "m m")
        .replace("nq", "ŋq")
        .replace("n q", "ŋ q")
        .replace("nc", "ŋc")
        .replace("n c", "ŋ c")
        .replace('l', "ɫ")
        .replace("ɫɫ", "ll")
        .replace("ɫi", "li")
        .replace("rh", "r̥")
        .replace('v', "w")
        .replace('x', "k͡s̺")
        .replace("ɡn", "ŋn")
        .replace('ë', "ɛ")
        .replace('ü', "u̞")
        .replace("ĕr", "ær")
        .replace("ie", "īe")
        .replace("cume", "kʉme")
        .replace("ptim", "ptɨm")
        .replace("crim", "krɨm")
        .replace(['c', 'q'], "k")
        .replace("um ", "ũː ")
        .replace("um,", "ũː,")
        .replace("um.", "ũː.")
        .replace("um?", "ũː?")
        .replace("um!", "ũː!")
        .replace("um·", "ũː·")
        .replace("em ", "ɛ̝̃ː ")
        .replace("em,", "ɛ̝̃ː,")
        .replace("em;", "ɛ̝̃ː;")
        .replace("em.", "ɛ̝̃ː.")
        .replace("em?", "ɛ̝̃ː?")
        .replace("em!", "ɛ̝̃ː!")
        .replace("em·", "ɛ̝̃ː·")
        .replace("ens", "ɛ̝̃ːs̺")
        .replace("ons", "ɔ̝̃ːs̺")
        .replace("er", "ær")
        .replace("ia", "ja")
        .replace("ii", "ji")
        .replace("io", "jo")
        .replace("iu", "ju")
        .replace("iā", "jā")
        .replace("iī", "jī")
        .replace("iō", "jō")
        .replace("iū", "jū")
        .replace("ea", "ia")
        .replace("ae", "ɐɛ̯")
        .replace("oe", "ɔɛ̯")
        .replace('i', "i̞")
        .replace('o', "ɔ")
        .replace('u', "u̞")
        .replace('y', "ʏ")
        .replace('a', "ɐ")
        .replace('e', "ɛ")
        .replace(['ā', 'á'], "ɐː")
        .replace(['ē', 'é'], "ɛ̝ː")
        .replace(['ī', 'í'], "iː")
        .replace(['ō', 'ó'], "ɔ̝ː")
        .replace(['ū', 'ú'], "uː")
        .replace(['ȳ', 'ý'], "yː")
        .replace('ă', "ɐ")
        .replace('ĕ', "ɛ")
        .replace('ĭ', "i̞")
        .replace('ŏ', "ɔ")
        .replace('ŭ', "u̞")
        .replace("y̆", "ʏ")
        .replace("ũ̞ː", "ũː")
        .replace("ɡw", "ɡʷ")
        .replace("kw", "kʷ")
        .replace("kh", "kʰ")
        .replace("ph", "pʰ")
        .replace("th", "tʰ")
        .replace(").", ".")
        .replace("),", ",")
        .replace(',', " ∣")
        .replace(';', " ∥")
        .replace(':', " ∣")
        .replace(". ", " ∥ ")
        .replace('.', "")
        .replace("! ", " ∥ ")
        .replace('!', "")
        .replace("? ", " ∥ ")
        .replace('?', "")
        .replace('(', "∣ ")
        .replace(')', " ∣")
        .replace('·', " ")
        .replace(" - ", " ∣ ")
        .replace(" – ", " ∣ ")
        .replace("--", " ∣ ")
        .replace("∣ ∣", "∣")
        .replace("∣ ∥", "∥");

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Classical Latin" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// LATIN: ORTHOGRAPHY

pub fn ortlat(original_text: &str, usefile: &str, outputfile: &str, reset: &str, red: &str, cyan: &str, yellow: &str) {
    let result = original_text
        .replace('a', "A")
        .replace('á', "Á")
        .replace('ă', "A")
        .replace('ā', "Á")
        .replace('b', "B")
        .replace('c', "C")
        .replace('d', "D")
        .replace(['e', 'ĕ', 'ë'], "E")
        .replace(['é', 'ē'], "É")
        .replace('f', "F")
        .replace('g', "G")
        .replace('h', "H")
        .replace(['i', 'ĭ'], "I")
        .replace(['Ī', 'ī', 'í', 'Í'], "ꟾ")
        .replace('j', "I")
        .replace('k', "K")
        .replace('l', "L")
        .replace('m', "M")
        .replace('n', "N")
        .replace(['o', 'ŏ'], "O")
        .replace('ō', "Ó")
        .replace('p', "P")
        .replace('q', "Q")
        .replace('r', "R")
        .replace('s', "S")
        .replace('t', "T")
        .replace(['u', 'v'], "V")
        .replace('ú', "Ú")
        .replace('ŭ', "V")
        .replace('ū', "Ú")
        .replace('ü', "V")
        .replace('w', "VV")
        .replace('x', "X")
        .replace('y', "Y")
        .replace("y̆", "Y")
        .replace(['ȳ', 'ý'], "Ý")
        .replace('z', "Z")
        .replace('Ă', "A")
        .replace('Ā', "Á")
        .replace(['Ĕ', 'Ë'], "E")
        .replace('Ĭ', "I")
        .replace(['Ī', 'Í'], "ꟾ")
        .replace('J', "I")
        .replace('Ŏ', "O")
        .replace('Ō', "Ó")
        .replace('Ŭ', "V")
        .replace('Ū', "Ú")
        .replace('Ü', "V")
        .replace("Y̆", "Y")
        .replace('Ȳ', "Ý")
        .replace(" - ", "·")
        .replace(" – ", "·")
        .replace("--", "·")
        .replace(", ", "·")
        .replace("; ", "·")
        .replace(": ", "·")
        .replace(". ", "·")
        .replace("! ", "·")
        .replace("? ", "·")
        .replace(['.', '!', '?'], "")
        .replace(' ', "·");

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("CLASSICAL LATIN:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("{}", cyan.to_owned() + "Classical Latin" + reset + ":");
        println!();
        print!("{yellow}");
        println!("{result}");
        print!("{reset}");
    }
}
